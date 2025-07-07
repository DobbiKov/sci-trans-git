# One shot translation with few changes

A key objective of this project is to preserve the user’s distinctive writing
and translation style. An essential component in achieving this goal is the
ability to leverage an existing translation database for reuse, enabling
consistency and stylistic continuity across translations.

The primary objective of this experiment was to investigate the translation
quality and the extent to which style is preserved when a large language model
(LLM) is presented with a source chunk alongside its translation and
subsequently tasked with translating a slightly modified version of the source
text.

## Problematic
Even though the translation database is implemented, it's primary and the only
one usage is to verify if a chunk of text has been changed since previous
translation session and if it was not, to retrieve its translation.

The problem comes when a chunk has been slightly changed (2-5 words) that
should not affect an output. However, the LLMs usually follow the source chunk
style of writing and not the translation one. Thus, even though the translation
has been edited by a human or even, possibly, completely rewritten, it isn't
counted or used during the retranslation process. For instance, this problem
can be encountered when a project is being translated on many languages with
different (human) translators that have different styles of writing.

## Setup
In order to evaluate its quality, the chunks of different sizes have been taken
from the SageMath book[^1] as it is bilingual and translated by humans and the
astronomy course[^2] due to the same reasons but without MyST or Markdown
syntax because the XML tagging usage was very desirable due to the syntax
preserving and translation quality it provides. 

The choice of models is:
- `gemini-2.0-flash`
- `llama-3.3-70B`

The Gemini model has been chosen due to the possibility of free usage, low cost
and low energy consumption. The llama model has been chosen due to its smaller
size comparing to the gemini model as well as its hostage on the Paris-Saclay
University servers and ease of use.

The models have been fed with a prompt provided in [annex](#prompt),
translation examples chosen from the sources listed above and slightly changed
source chunk.

## Observations
When providing small or medium size chunks (1-3 paragraphs long), the two
models provide state-of-the-art results with high quality translation and
styling preserving.

With big chunks (4+ paragraphs) `gemini` provides medium results, sometimes
losing in the translation quality while `llama` doesn't retranslate the chunk
and provides the given translation from the example.

An interesting aspect of this study is the use of an unrelated example, i.e.,
when the example chunk and its translation share no semantic or structural
similarity with the source chunk to be translated. Notably, Gemini produced
results comparable to a zero-shot prompt in this setting, whereas LLaMA
performed poorly, generating translations with grammatical errors and incorrect
article usage.

## Conclusion
This study highlights the challenges and opportunities in leveraging large
language models for maintaining stylistic consistency in translation workflows,
particularly when minor edits are introduced to the source text. The
experiments demonstrate that both Gemini and LLaMA perform well on small to
medium chunks, preserving style and producing high-quality translations.
However, when handling larger chunks or when provided with unrelated examples,
notable divergences emerge: Gemini exhibits a gradual decline in quality, while
LLaMA struggles to adapt and often defaults to reusing existing translations
without sufficient re-evaluation.

These findings suggest that while current LLMs are capable of supporting
workflows requiring minor source modifications, they are limited in their
ability to fully integrate pre-existing human-edited translations as stylistic
anchors. Addressing this limitation would require refining prompt engineering
strategies and potentially integrating mechanisms to enforce alignment with
translation databases. Future work could explore hybrid approaches, combining
LLM outputs with post-editing tools and stylistic feedback loops, to ensure
greater consistency and fidelity across multilingual projects.


## References
[^1]: https://books.google.fr/books/about/Computational_Mathematics_with_SageMath.html?id=v8iBDwAAQBAJ&source=kp_cover&redir_esc=y
[^2]: https://m2-npac-ac.pages.in2p3.fr/chapters/fr/start

## Annex
### Prompt
```
You are tasked with updating the translation of a scientific document from [SOURCE_LANGUAGE] to [TARGET_LANGUAGE] using a structured XML format.

The document consists of <TEXT> elements that contain translatable content (sentences or paragraphs), interleaved with <PH> tags that represent non-translatable content such as LaTeX commands, math expressions, or code.

### Context:
You are provided with:
1. The original source paragraph (in [SOURCE_LANGUAGE]).
2. Its correct translation (in [TARGET_LANGUAGE]).
3. A **new version of the source paragraph**, which differs only slightly (1–3 words changed).

### Your task:
- **Update the translation** to reflect the changes in the new source.
- **Reuse as much as possible** from the original translation.
- Keep the XML structure unchanged, including all <PH> tags and their attributes.

### Rules:
- Translate or modify **only the parts that changed** in the new source.
- Do **not modify, remove, or reorder** any <PH/> tags.
- Use the `original` attribute of each <PH/> tag for understanding grammar context (e.g. case, gender, plurality), but do **not translate or alter** their content.
- Your output must contain **only** the updated XML <TEXT> block — no explanations, comments, or extra markup.
- All <PH> tags must be self-closing and written in the form:
    <PH id="..." original="..."/>
- Never use closing tags like </PH> or wrap content inside <PH> tags.
- If the provided chunk doesn't contain any <PH> tags, you simply translate the text inside the <TEXT> tag and return it in the initial format

### Output Format:
<document>
<TEXT>
  ...translated text with embedded <PH id="..." original="..."/> tags...
</TEXT>
</document>

Don't cover the output in any Markdown or XML environments like (```) etc. 

### Provided Input:

#### Old Source:
[OLD_SRC]

#### Old Translation:
[OLD_TGT]

#### New Source:
[NEW_SRC]

Now provide the updated translation:
```

