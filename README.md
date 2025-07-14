[![DOI](https://zenodo.org/badge/990720286.svg)](https://doi.org/10.5281/zenodo.15775111)
# Document translation tool
This is a main repository of the translation tool. You should visit [CLI repo](https://github.com/DobbiKov/translate-dir-cli) if you want to translate your document using the CLI and [LIB repo](https://github.com/DobbiKov/translate-dir-lib) if you are interseted in using the translation library.

## TL;DR
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



## 📚 Citation

If you use this software in your research, please cite it as follows:
```bib
@software{korotenko-sci-trans-git,
    author = {Yehor Korotenko},
    title = {sci-trans-git},
    year = {2025},
    publisher = {GitHub},
    version = {0.2.0-alpha},
    url = {https://github.com/DobbiKov/sci-trans-git},
    doi = {10.5281/zenodo.15775111}
}
```

## The state of the project
In this section the state of the project is described as well as the reports that are done by the moment of the README edit.

### State of the project
Currently, we still explore different models as well as their capabilities in
translation and syntax, layout and structure preserving.

In parallel, we develop a prototype library to simplify the _project_
translation (such as `LaTeX` document splited into multiple files, or `myst`
project). 

#### Library
For now, the library translates jupyter notebooks by using `jupytext`
module to extract contents of cells, pass it to the models and extract
translation and `LaTeX` documents by using `pylatexenc` to construct an `AST` and
divide the document into chunks and then translate those chunks.
The translation is stored in the translation database, in order to
not retranslate the translated chunks and just retrieve them.

The library also provides a possibility to correct translation (i.e rewrite or
fix the translated file by the model and save the translation in the database
so it is not overwritten in the future).

In order to improve translation quality and avoid ambiguity the vocabulary feature 
is provided in the library. For the translation command, there's an optional 
parameter that is a vocabulary (translation pairs) that would help the model 
to use the appropriate words and phrases that are presented in the vocabulary.

#### CLI
In order to simplify the library testing and presentation, a CLI application
that implements library functionality has been developed.

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
4. [Translation evaluation report](./reports/report4_translation_evaluation.md) = 
the report that evaluates the quality of translation of different models
such as: `gemini`, `llama` and `gemma` from and to languages such as:
- English
- French
- German
- Ukrainian
5. [Translation evaluation tool report](./reports/report5_evaluation_tool.md) = the 
report presents the new tool for automatic translation evaluation using popular
metrics.
6. [Latex chunking report](./reports/report6_latex_chunking.md) = the report
presents the results of the work on developing the feature of dividing `LaTeX`
documents into chunks in order to simplify translation.
7. [New ways of passing text to LLMs report](./reports/report7_different_translation_ways.md) 
= the report presents the results of the work on exploring the new way of passing text
into LLMs in order to improve the reliability of structure preserving and reduce number of tasks
that the models must handle simultaneously.
8. [One shot translation to preserve writing style report](./reports/report8_retranslation_with_few_changes.md) = the report presents the explorations and the results about preserving the style using one-shot prompting technique.

## Resources presented in the repository
- The `prompt` directory contains files that store prompts that are currently
  used in the library to translate documents.
- [library itself](https://github.com/DobbiKov/translate-dir-lib/)
- [CLI library implemetation](https://github.com/DobbiKov/translate-dir-cli/)
- [translation evaluation tool](https://github.com/DobbiKov/translation-evaluator/) 
    for automatic translation evaluation using reference translations.

## Current development direction
### High priority direction
1. When an untranslated chunk encountered it is possible that this chunk has been translated except one/two words has been changed/added/removed. Thus, it would     be preferable to get as close result as it was except with taking changes into account. Thus, the ways to extract old chunk, it's translation and use them to     get the new translation as close to the previous one as possible.
2. Explore the ways to use the translation database and to provide the model
   the way and the style it should write the translation in.
3. Explore and implement an "*RAG*" for the
   vocabulary, that is to say extract only essential translation pairs from the
   vocabulary and pass them to the prompt in order to reduce the number of
   tokens.

### Low priority direction
Add the chunking support for `markdown`, `myst` and the general text files.
Implement `xml` tagged conversion and translation for `LaTeX` and `jupyter notebook`
