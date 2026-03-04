# Dual-model fallback and XML retry strategy

## Problematic

During translation of scientific documents, standard LLMs occasionally produce
malformed XML output. In the XML-based translation workflow, the model is
expected to return a well-formed `<document><TEXT>...</TEXT></document>` structure
with `<PH>` placeholder tags intact. When the model deviates from this format
— for example by generating unescaped characters, broken tags, or truncated
output — the response cannot be parsed by the `xml.etree.ElementTree` parser,
raising an `ET.ParseError`.

A single transient failure of this kind should not abort the translation of the
whole file, nor should it cause the chunk to be permanently skipped without a
second attempt. At the same time, some malformed outputs are not random: they
reflect a structural limitation of the model that a second call with the same
model is unlikely to correct. A more capable reasoning model, given its stronger
instruction-following abilities, is a better fallback in that case.

## Solution

### Three-attempt retry chain

Each chunk translation now goes through up to three attempts before being marked
as untranslated:

1. **Attempt 1** — standard model. On success, the result is used directly.
2. **Attempt 2** — standard model again. Triggered only if attempt 1 produced a
   broken XML (`ET.ParseError`). Covers transient generation failures.
3. **Attempt 3** — reasoning model. Triggered only if attempt 2 also raised
   `ET.ParseError`. If no reasoning model is configured, the standard model is
   used again.

Any non-XML exception (e.g. `ApiCallError`, `ModelOverloadedError`) at any
attempt is caught immediately and the chunk is marked as `ChunkTranslationFailed`
without further retries, so API-level errors do not trigger the XML-specific
retry path.

If all three attempts fail, the chunk is recorded as untranslated and the file
translation continues with the next chunk.

### Configuration

The reasoning model is optional and stored independently of the standard model
in the project configuration:

```yaml
llm_service: ilaas
llm_model: meta-llama/Llama-3.3-70B-Instruct
llm_reasoning_service: anthropic
llm_reasoning_model: claude-sonnet-4-6
```

Both fields can be set via the CLI:

```bash
# Set the standard model
translate-dir set-llm ilaas llama-3.3-70b

# Set the reasoning model (optional)
translate-dir set-reasoning-model ilaas gpt-oss-120b
```

The current configuration, including whether a reasoning model is set, is
visible in `translate-dir info`.

## Observations

The retry mechanism is most useful with medium-sized open models such as
`llama-3.3-70B`, which follow the XML prompt well in the majority of cases but
occasionally produce structurally broken output. Larger closed-source models
(`gemini-2.5-flash`, `claude-sonnet-4-6`) very rarely trigger even the first
retry.

Reasoning models are significantly more reliable at respecting structured output
constraints. In the cases where `llama-3.3-70B` failed twice with broken XML,
`gemini-2.5-flash` as a fallback produced a correct and well-formed response in
all observed instances, whereas `gpt-oss-120b` preserved XML structure but left 
several sentences in untranslated.

The additional latency of the third attempt is negligible at the file level,
since XML failures represent a small fraction of all chunks.

## Conclusion

The dual-model retry chain provides a practical reliability improvement for
production translation runs. Transient XML failures are absorbed by the second
attempt with the standard model, while structurally persistent failures are
escalated to a reasoning model that is more capable of following strict output
formatting requirements. In both cases the file translation is never aborted due
to a single chunk failure: the worst outcome is that one chunk remains untranslated
and is flagged for manual review.

The feature is fully optional: if no reasoning model is configured, the third
attempt simply uses the standard model again, preserving the previous behaviour
with minimal overhead.
