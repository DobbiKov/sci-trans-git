# Library development progress: March 2026

This report summarises the major features, bug fixes, and improvements introduced
to `translate-dir-lib` during March 2026. The changes span four broad areas:
Typst document support (initial version and follow-up improvements), a rewritten
MyST parser, smarter `needs_review` tagging, and infrastructure improvements.

---

## Typst document support

### Problematic

The library supported Jupyter notebooks, LaTeX, and MyST/Markdown. Typst is an
emerging document format increasingly used for scientific and technical writing,
and its absence was a gap for projects that have already migrated away from LaTeX.

### Solution

A first-class Typst backend was introduced across ten files:

- `typst_chunker.py` — splits a Typst source file into translatable chunks at
  paragraph and block boundaries, with awareness of nested scopes.
- `typst_file_translator.py` — drives per-chunk translation and writes the
  translated output back to disk.
- `xml_manipulator_mod/typst.py` — converts Typst tokens to the internal XML
  representation and reconstructs the Typst source from translated XML.
- Cache syncing for Typst was wired into the existing cache infrastructure so
  that incremental re-translation works out of the box.

A dependency on `typst-syntax` was added to `pyproject.toml` for Typst source
parsing.

### Subsequent improvements (PR #105)

After the initial version, several follow-up improvements were merged:

**Subchunking for large chunks.** The Typst chunker now detects chunks that
exceed the token budget and further splits them at sentence boundaries (via the
`translator_retrieval` subchunker). This prevents oversized chunks from being
sent to the LLM in a single call and avoids truncation artefacts.

**Python documentation support.** The retrieval layer was extended to support
Python-style docstrings as a source format, expanding the set of document types
that can feed into the translation pipeline.

**Bug fix: command splitting across chunk boundaries.** A bug caused a single
Typst command to be split across two consecutive chunks in some cases, producing
malformed output. The chunker boundary logic was tightened to prevent this.

**Linux support.** The `typst-syntax` dependency was upgraded from v0.1.5 to
v0.1.6, which includes pre-built wheels for Linux. Previously, Linux users had
to compile the extension from source.

### Supported Typst constructs

- Paragraphs, headings, and block-level content
- Commands (function calls) with configurable translatable parameters — the
  project configuration can specify which parameters of which functions contain
  translatable text
- Strings inside math environments
- Smart quotes (correctly preserved round-trip)
- Nested scopes and role contents (including `definiendum`)
- `<text>` nodes inside role bodies

---

## MyST parser rewrite (PR #100)

### Problematic

The original MyST XML manipulator had grown organically and contained several
correctness issues: imprecise whitespace handling, incorrect nesting of lists
inside admonitions, lost content in certain directives, and a tendency to merge
consecutive `\n` newlines into a single placeholder in ways that broke the
round-trip.

### Solution

The MyST XML manipulator (`xml_manipulator_mod/myst.py`) was substantially
rewritten — approximately 1 150 lines reworked across 514 additions and 651
deletions. Key correctness improvements:

**Exact whitespace and tab preservation.** The rewrite treats indentation and
inline spacing as exact, character-level data rather than normalising it. A
dedicated test verifies that round-tripping any MyST file through parse →
translate → reconstruct produces byte-identical indentation.

**`\n` as text, not placeholder.** A bare newline inside a paragraph is now
tagged as translatable text rather than a placeholder. This avoids the previous
behaviour where trailing newlines were silently dropped or merged.

**No merging of consecutive newline placeholders.** The XML marker no longer
collapses two adjacent placeholders when one of them is a `\n` token, preserving
the blank-line structure of the source document.

**Improved nesting.** Lists nested inside admonitions and headings are now
handled correctly. Previously, a list inside an `.. admonition::` block could
lose its items or produce duplicate text.

**Math directive fixes.** The `{eval-rst}` and `{amsmath}` directives
previously lost their body content when parsed. This was fixed so that their
content is preserved as an opaque placeholder.

**Expanded placeholder-only directives.** The following directive types were
added to the placeholder-only list (their bodies are not sent to the LLM):
`{versionadded}`, `{versionchanged}`, `{deprecated}`, `{toctree}`,
`{tab-set}`, `{table}`, `{todo}`, `{TODO}`, and `{list-table}`. Tests were
added for each.

**`<text>` translatable inside roles.** MyST roles can contain arbitrary inline
content. The XML manipulator was updated so that `<text>` nodes nested inside
role content are marked as translatable rather than being treated as opaque
placeholders.

**Definiendum role contents translatable.** The `definiendum` role (used for
terms being defined) had its inner text marked as non-translatable by default.
This was corrected so that its contents are now translatable like any other
inline text.

---

## Smarter `needs_review` tagging (PR #102)

### Problematic

Every translated chunk, regardless of whether the LLM was actually called,
received the `needs_review` metadata tag. This meant that placeholder-only
chunks (which are copied verbatim without any LLM call) were incorrectly
flagged for human review, producing noise in review workflows.

A second bug caused incorrect tag assignment when the same chunk appeared more
than once in a file: the first occurrence received `needs_review` but later
occurrences did not, because the deduplication logic ran before the tag was
applied.

### Solution

**LLM-conditional tagging.** The `needs_review` tag is now added to a chunk's
metadata only when the LLM was actually invoked to produce the translation. All
four file translators (LaTeX, MyST, Notebook, Typst) were updated consistently.
Placeholder-only chunks that are copied from cache or skipped entirely no longer
receive the tag.

**Duplicate chunk fix.** The tag-assignment logic was moved so that it runs
independently for each occurrence of a chunk, rather than once per unique chunk
text. All duplicate occurrences are now tagged correctly.

**Metadata reading from target file.** A `cache_rebuilder` extension was added
that reads an already-translated target file and extracts per-chunk metadata
(including existing `needs_review` tags) from it. This allows the cache to be
seeded from existing translations without losing previously set metadata.

---

## Infrastructure improvements

### GitHub Actions CI

A GitHub Actions workflow (`.github/workflows/tests.yml`) was added to run the
full test suite automatically on every push and pull request. This is the first
continuous-integration setup for the project and catches regressions before
merge.

### Armenian language support

`hy` (Armenian) was added to the `Language` enum, making it a valid target
language for the `translate-dir` CLI alongside the existing set.

---

## Conclusion

March 2026 marked the introduction of Typst as a first-class document type,
joining Jupyter notebooks, LaTeX, and MyST Markdown. The MyST parser was
substantially rewritten to fix a class of whitespace and nesting correctness
issues. The `needs_review` tagging system was made precise so that only chunks
that were actually processed by the LLM are flagged, reducing noise in review
workflows. Finally, continuous integration was set up and Armenian was added as
a supported target language, broadening the tool's reach.
