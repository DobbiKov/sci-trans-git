# Document translation tool
We explore the specific challenges of authoring and maintaining multilingual
computational scientific narratives, like course notes, textbooks, or reference
manuals, and the design space for leveraging adaptive machine translation to
assist authors.

## Motivation and concept

With the advancement of automated translation, there now exists plethora of
tools and services for translating documents. These tools are well suited for
one shot translations: author in one language; machine translate; proofread and
postedit. Consider now a large document that evolves over a long period of
time; say course notes that one wants to provide in, e.g. French and English,
and maybe some other language. The above workflow is the not suitable anymore:

1.  The high value human effort of proofreading -- in particular in terms of
    choice of style and terminology -- is lost at each iteration.
2.  The authors may want to alternatively improve the document in one or the
    other language, and propagate the improvements to the other languages.

Instead, one wants workflows where changes in one language can be propagated to
the other languages, not only leaving the rest of the text unchanged, but
exploiting it has a source of aligned chunks of translated and proofread texts
to guide the style and terminology of the translation (Adaptive Machine
Translation). Also a seamless integration in the authoring environment and
workflow is desirable.

With the advent and large scale availability of (adaptative) machine
translation, LLM's, few shot learning, RAGs, time should be ripe to leverage
that technology to have open source, sovereign, and privacy preserving tools
supporting such workflows in the authors own authoring environment for, e.g.,
course notes written in some markup language like MyST/Markdown/Jupyter or
LaTeX, collaboratively authored using a software forge. Either by adopting and
deploying existing systems, or building a lightweight one from existing bricks.

## The state of the project
In this section the state of the project is described as well as the reports that are done by the moment of the README edit.

### State of the project
Currently, we still explore different models as well as their capabilities in
translation and syntax, layout and structure preserving.

In parallel, we develop a prototype library to simplify the _project_
translation (such as `LaTeX` document splited into multiple files, or `myst`
project). 

For now, the library translates only jupyter notebooks by using `jupytext`
module to extract contents of cells, pass it to the models and extract
translation. The translation is stored in the translation database, in order to
not retranslate the translated chunks and just retrieve them.

The library also provides a possibility to correct translation (i.e rewrite or
fix the translated file by the model and save the translation in the database
so it is not overwritten in the future).

### Reports
1. [First report](./reports/report1.md) = the report presents the first
   prototype written in _rust_ programming language presenting the idea of the
   tool. The report also provides useful information about the energy
   consumption of the existing high-performance models.
2. [Second report](./reports/report2.md) = the report presents the version of
   the library and the tool on python as well as with the first version control
   prototype.
3. [Aristote evaluation report](./reports/report3_test_aristote.md) = the
   report provides the comparison of the translations of the `gemini` and
   `llama-3.3` models.

## Resources presented in the repository
- The `prompt` directory contains files that store prompts that are currently
  used in the library to translate documents.
- [library itself](https://github.com/DobbiKov/translate-dir-lib/)
