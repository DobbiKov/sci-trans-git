# Library development progress: July 2025 – February 2026

This report summarises the major features, architectural changes, and
improvements introduced to `translate-dir-lib` between July 2025 and
February 2026. The changes span six broad areas: MyST/Markdown document
support, a configurable LLM backend, translation robustness improvements,
CLI integration, a reworked cache system, and a set of developer-experience
improvements.

---

## MyST / Markdown document support

### Problematic

The library already provided chunking and translation for Jupyter notebooks
(cell-based) and LaTeX documents. MyST Markdown is the primary authoring
format for many scientific projects built on the Jupyter Book ecosystem, and
its absence was a significant gap. Unlike LaTeX, MyST has no built-in
cell boundary — the document must be segmented into translatable chunks by
the library itself.

### Solution

A MyST parser was built on top of the `markdown-it-py` library, which
tokenises MyST/Markdown source into a stream of typed tokens. The token
stream is walked to reconstruct the original markup, and the document is
divided into chunks at paragraph, heading, and block-level boundaries.

The following MyST/Markdown constructs are handled:

- Paragraphs, headings, and block-quotes
- Ordered and unordered lists (with correct indentation)
- Fenced code blocks and `code_inline` spans
- Admonitions (directives), including `todo` admonitions
- Footnotes
- HTML blocks
- `myst_block_break` and inline MyST comments

`\n` handling was divided into two categories (hard and soft breaks) to
avoid producing spurious blank lines in the reconstructed output. A
round-trip test script was added to verify that parsing and reconstruction
are lossless.

Because MyST files and plain Markdown share the same extension, the library
no longer misclassifies `.md` files as Jupyter notebooks.

---

## Configurable LLM backend

### Problematic

The library originally called the Google Gemini API directly and relied on a
single hardcoded environment variable (`GOOGLE_API_KEY`). This made it
impossible to use alternative providers or to run the tool on infrastructure
that hosts different models.

### Solution

The `unified-model-caller` library was introduced as the single interface to
all LLM providers. Translation calls are now routed through this abstraction
layer, and the project configuration stores two fields:

```yaml
llm_service: ilaas
llm_model: meta-llama/Llama-3.3-70B-Instruct
```

The environment variable was renamed from `GOOGLE_API_KEY` to `LLM_API_KEY`
to reflect its provider-agnostic role. Both fields are settable via the CLI:

```bash
translate-dir set-llm <service> <model>
```

Available services can be listed at any time with:

```bash
translate-dir list-services
```

The per-model cooldown between API calls, previously a fixed five-second
wait, is now derived from the `unified-model-caller` library and is specific
to each model, avoiding unnecessary delays for models that do not require
them.

---

## Translation robustness improvements

### Placeholder-only chunk optimisation

Some document chunks consist entirely of non-translatable content (LaTeX
commands, code, placeholders). Sending such chunks to the LLM wastes tokens
and API quota without producing any useful output. The translator now
detects these *placeholder-only* chunks before making an API call and skips
them entirely. Detection events are recorded in the trace log. Tests cover
the detection logic and the end-to-end skip behaviour.

### Continuing translation on chunk error

Previously, an exception raised while translating a single chunk would
propagate and abort the translation of the entire file. The translator now
catches per-chunk errors, records the chunk as failed, and continues with
the remaining chunks. This ensures that a single problematic chunk does not
prevent the rest of the file from being translated.

---

## CLI integration

### Problematic

The command-line interface was maintained in a separate repository, creating
friction for contributors and making it difficult to keep the CLI in sync
with library changes.

### Solution

The CLI was moved into this repository. Documentation covering all available
commands was added to both the README and a dedicated CLI reference. The
tool can be reinstalled after source changes using a provided shell script.

---

## Cache system

### Rename: `db` → `cache`

The internal translation database was renamed from `db` to `cache`
throughout the codebase — source code, tests, and documentation — to better
reflect its role as a local cache of translation results rather than a
persistent application database. The `update-translation` command was
removed and superseded by `cache sync`.

The database structure was also extended: chunks are now stored per source
file rather than in a single flat collection, which improves lookup
performance and makes per-file cache operations straightforward.

### Cache management commands

A `cache clear` command was introduced with a range of flags for targeted
invalidation:

| Flag | Effect |
|---|---|
| *(none)* | Interactive clear |
| `--all` | Clear the entire cache |
| `--file <path>` | Clear cache for a specific source file |
| `--lang <code>` | Clear cache for a specific target language |
| `--keyword <word>` | Clear entries whose source text contains a keyword |
| `--missing-chunks` | Remove entries whose corresponding source chunk no longer exists, including orphan target-language chunks |

A `cache rebuild` command was also added to reconstruct the cache from the
translated files on disk, useful after manual edits or repository cloning.

All new commands are covered by tests.

### Bug fix: CLI path parameter

A bug in the `cache clear` command caused the path parameter to be ignored
when passed via the CLI. This was corrected and a regression test was added.

---

## Verbose mode and user-visible warnings

### Problematic

Warnings and non-fatal errors produced during translation were only visible
when the tool was invoked with a debug flag, leaving users unaware of
potential issues in normal operation.

### Solution

A `--verbose` mode was added for detailed diagnostic output. Independently
of verbose mode, warnings and errors are now always printed to the user.
This includes notifications about chunks that failed to translate, chunks
that were skipped for being placeholder-only, and cache inconsistencies.

---

## Relative paths in configuration

### Problematic

The project configuration stored absolute paths, making it non-portable:
a project initialised on one machine or under one user account would fail
on any other.

### Solution

The configuration now stores all paths in relative form. Any absolute path
found in an existing configuration file is automatically rewritten to a
relative path on the next access. Tests covering relative path handling were
added to the test suite.

---

## Performance: startup time

Importing `project_manager` caused a cascade of heavy imports that
noticeably delayed the CLI startup, even for commands that do not require
translation (e.g. `info`, `list-services`). The module was split into two
files and imports were restructured so that heavy dependencies are only
loaded when needed. The `unified-model-caller` library was simultaneously
updated to a version with an improved import time. Together these changes
bring the CLI startup time to a level comparable to other Python CLI tools.

---

## Conclusion

Over this period the library evolved from a Jupyter-and-LaTeX-only tool with
a hardcoded API backend into a more general-purpose translation framework.
MyST/Markdown is now a first-class document type, the LLM backend is fully
configurable, and the cache system provides fine-grained control over
stored translations. The CLI integration reduces the maintenance burden, and
the robustness improvements ensure that individual chunk failures or API
errors do not abort an entire translation run. Startup time and path
portability improvements make the tool more practical for day-to-day use
across different environments.
