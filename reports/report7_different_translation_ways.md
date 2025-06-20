# Trying different translation ways with small models
The Large Language Models have been developed and slightly improved in recent
years and incremented the number of tasks it can do instead of humans. One of
such tasks is translation. In our project, we are interested not only in
translation itself, but also in preserving syntax, structure and layout.
According to previous reports, `gemini-2.0-flash` excels in this task. However,
it is still not reliable in 100% and makes mistakes such as replacing one
character on another.

Even though _gemini_ provides amazing results, it is still close sourced model
that can't be accessed using **proprietary** API. Thus, this report provides exploration of the capabilities of
smaller open source/weighted models such as `gemma3-27b` and `llama-3.3-70b`. 

## Models and services used 
- `gemma3-27b` model has been accessed though _google genai API_. Even though
  it is a proprietary service, it provides a free access to the open-weighted
  model that can be used and hosted by anyone.
- `llama-3.3-70b` is accessed though the _aristote_ service provided by _Université Paris-Saclay_.

## Ideas and hypothesis
The main point of this report was the effectiveness and possibility to use XML
tags for chunk translation and structure preserving.

### Motivations
When a model is given any text written using a markup language such as LaTeX or
Markdown and ask to translate it, it has many tasks and ambiguities. That is to say,
the model has to: 
- identify all the syntax parts (can be ambiguous, especially when a code snippet is provided)
- identify natural language parts
- build context
- translate the text
- decide the new order

Small models are not capable of handling this number of tasks that is not definite.

### XML tags
In order to make the task for models more clear, XML tags are suggested to use.
More precisely, XML tags are used in a way to mark and distinguish _Natural
Language_ and _Syntax Parts_ for the model that would reduce ambiguities and
the number of tasks that a model has to handle.

For the XML code only two tags are used:
- `<TEXT id="..">..</TEXT>` that marks a human text
- `<PH id=".." _/>` that marks non-human text (usually specific syntax parts)

## XML usage styles
### All non-human-text removal and human-text tag coverage
The first idea is to remove all the non-human-test parts and replace them by
_placeholder_ tag that would help to reconstruct the markup version of the text
after the translation. And cover all the human-text parts by `<TEXT>` tag.

Example:
- original LaTeX
    ```tex
    Soit $A \subset E$ borné si $\exists R > 0$ tel que $A \subset B(x, R)$.
    ```
- XML version:
    ```xml
    <TEXT id="2">Soit </TEXT><PH id="3"/><TEXT id="4">
    borné si </TEXT><PH id="5"/><TEXT id="6"> tel que
    </TEXT><PH id="7" /><TEXT id="8">.</TEXT>
    ```

In most cases the absence of syntax-parts removes the context that the text is
written in. That leads to low-quality translation.

### Providing original source-code 
In order to provide the context, the idea of providing the source code has been
investigated.

Example of a prompt:
````
Act as a French to English translator. You are given LaTeX source code as well
as XML code. Use the LaTeX code to grasp the context, then translate the text
written using XML code and return the translated version using XML code.

```tex
Soit $A \subset E$ borné si $\exists R > 0$ tel que $A \subset B(x, R)$.
```
```xml
<TEXT id="2">Soit </TEXT><PH id="3"/><TEXT id="4">
borné si </TEXT><PH id="5"/><TEXT id="6"> tel que
</TEXT><PH id="7" /><TEXT id="8">.</TEXT>
```
````

However, providing both source code and XML code doubles the amount of code to
provide. Due to the small context window of small models that leads to
undefined behaviors such as:
- untranslated parts
- low quality translation
- usage of languages that are not asked in the prompt

This forces to abandon this idea.

### Provide the syntax in `<PH>` tags
In order to provide the context of all the chunk but avoid providing source
code, the idea of providing source non-human text as a property of
`<PH>`(placeholder) tag has been investigated.

Example:
- original LaTeX
    ```tex
    Soit $A \subset E$ borné si $\exists R > 0$ tel que $A \subset B(x, R)$.
    ```
- XML version:
    ```xml
    <TEXT id="2">Soit </TEXT><PH id="3" original="$A \subset E$"/><TEXT id="4">
    borné si </TEXT><PH id="5" original="$\exists R > 0$"/><TEXT id="6"> tel que
    </TEXT><PH id="7" original="$A \subset B(x, R)$"/><TEXT id="8">.</TEXT>
    ```

This approach slightly improved the translation quality due to the presence of
the context of the chunk. In our example, model grasps that in this sentence we
introduce a math operator and certain conditions.

However, as all the human-text parts are covered by `<TEXT>` tag, the model
cannot reorder the text parts from different `<TEXT>` tags. Example of such problem:
Let's introduce an other latex chunk:
- original LaTeX
    ```tex
    Intersection \underline{finie} des ensembles ouverts est ouvert.
    ```
    In English: The finite intersection of open sets is open.
- XML version:
    ```xml
    <TEXT id="1">Intersection </TEXT><PH id="1" original="\underline{"/><TEXT id="2">
    finie</TEXT><PH id="2" original="}"/><TEXT id="3"> des ensembles ouverts est ouvert</TEXT>
    ```
The problem is, that in French an adjective goes after the noun, unlike in
English where the adjective goes before the noun. The translation from our
example requires to change the order of `finie` and `intersection` but the
model can't do so due to the way the human-text parts are covered in the `<TEXT>` tag.
This leads to incorrect translation.

### Use only one `<TEXT>` tag
The last idea has been to use only one text tag that would contain human text and non-translatable parts covered in `<PH>` tag, example:
- original LaTeX
    ```tex
    Intersection \underline{finie} des ensembles ouverts est ouvert.
    ```
    In English: The finite intersection of open sets is open.
- XML version:
    ```xml
    <TEXT id="1">Intersection <PH id="1" original="\underline{"/>
    finie<PH id="2" original="}"/> des ensembles ouverts est ouvert</TEXT>
    ```

This allows the model to reorder words during the translation if needed. This
improved the translation quality and the text started to sound natural.

Faced problems: in less popular languages such as Ukrainian, in some cases the
model rearranged text in the way that some LaTeX macros (here: underline)
contained text the should be outside.

## Other observations
Gemma model is still very unreliable for syntax preserving. One of the possible
reasons is the small number of parameters (27B) that doesn't allow the model to
handle all the context. Thus, this modal is not usable for our needs.

However, the model shows far better results in terms of text writing especially
in Ukrainian. Its translations look more natural and recent.

## Future work
The future work should aim to explore the context limits of middle-sized models
such as `llama-3.3-70B` in order to understand its limitations and how much of
text or other context we can provide and get desirable results.
