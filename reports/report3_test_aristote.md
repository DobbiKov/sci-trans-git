# Results of the translations using different models

## Introduction 
For many laboratories and learning centers or even companies that want to
translate their documents the possibility of having their own model and using
it to translate documents is important and even required sometimes. 

To explore such possibility and the capabilities of open-weighted models the
translation tests have been conducted and thus presented in this report.

## Model choices
For the prototype of the _translation library_ the `gemini-2.0-flash` model has
been used due to the ease of use, thus all the results will be compared with
the translation outputs of this model.

As an open-weighted (in this case open source) model has been chosen
`llama-3.3-70B-instruct` as this model is used and provided for free and
educational use in the _Paris-Saclay University_.

## Testing data and first steps
Before conducting the evaluations, the python course has already been
translated using the `gemini-2.0-flash` model, thus the decision to translate
multiple notebooks of the same course has been made because there were already
translated output to compare with.

For the first attempt, three notebooks have been retranslated and an error
pattern has been discovered immediately, due to this reason the next tests and
the results are presented: 

## Tests
The prompt that is given to the models before each request is given in the [prompt](#prompt) section

Note: even though it is not related to the report, but the `gemma-3-27b` results are added as well.

#### cell 1

```py
### BEGIN SOLUTION
debut()
while( regarde() == Vide ):
    avance()
ouvre()
### END SOLUTION
```

Results:
- aristote: translates all the function names: **incorrect**
- gemma-3-27b: removes begin/end solution lines **incorrect**
- gemini-2.0-flash: doesn't translate: **correct**
- gemini-2.5-flash: doesn't translate: **correct**

---
#### cell 2

```py
animaux
```

Results:
- aristote: translates the variable `animaux`: **incorrect**
- gemma-3-27b: doesn't translate: **correct**
- gemini-2.0-flash: translates the variable `animaux`: **incorrect**
- gemini-2.5-flash: doesn't translate: **correct**
---
#### cell 3

```py
def somme_entiers_pairs(n): # c'est une fonction tres important
    i = 2
    somme = 0
    while i < n:
        if i % 2 == 0:
            somme = somme + n
        i = i + 1


    return somme # renvoie une somme
somme_entiers_pairs(10)
```

Results:
- aristote: doesn't translate code, translates comments: **correct**
- gemma-3-27b: doesn't translate code, translates comments: **correct**
- gemini-2.0-flash: doesn't translate code, translates comments: **correct**
- gemini-2.5-flash: doesn't translate code, translates comments: **correct**
---
#### cell 4

```py
assert est_gagnant()
```

Results:
- _aristote_: translates `est_gagnat` to `is_winner`: **incorrect**
- _gemma-3-27b_: translates `est_gagnat` to `is_winner`: **incorrect**
- _gemini-2.0-flash_: doesn't translate anything: **correct**
- _gemini-2.5-flash_: doesn't translate anything: **correct**
---
#### cell 5

```py
debut()
droite()
avance()
gauche()

if regarde() == Toile:
    gauche()
    avance()
    avance()
    droite()
    avance()
    avance()
    droite()
    avance()
    gauche()
else:
    avance()
    avance()
    gauche()
    avance()
    droite()

ouvre()
```

Results:
- _aristote_: translates all the function names: **incorrect**
- _gemma-3-27b_: doesn't translate anything: **correct**
- _gemini-2.0-flash_: doesn't translate anything: **correct**
- _gemini-2.5-flash_: doesn't translate anything: **correct**

### prompt
````
You are a specialized translation assistant. Your **PRIMARY AND SOLE TASK** is to translate **ONLY the natural language content** found **EXCLUSIVELY within comments and string literals** from a source language into **English**.

You **MUST** preserve **ALL OTHER PARTS of the input code cell content EXACTLY AS-IS**. This includes, but is not limited to:
*   **ALL code syntax** (keywords, operators, delimiters).
*   **ALL identifiers** (variable names, function names, class names, module names, argument names).
*   **Structure, layout, indentation, and whitespace.**
*   **Numerical and boolean literals.**
*   **Any errors present in the original code.**

**ABSOLUTELY DO NOT TRANSLATE, ALTER, OR MODIFY ANY CODE SYNTAX, IDENTIFIERS (like variable or function names), OR CODE STRUCTURE.** Your focus is strictly on natural language within comments and strings.

You must treat the input as a **raw source file**, not as a renderable or executable document.
Do **not** alter or correct formatting, layout, or syntax in any way.

## Input format
The code cell content to be translated will be wrapped inside a <document> tag, like this:
<document>
[original code cell content here]
</document>

Optionally, you may also receive a custom vocabulary dictionary wrapped in a <custom_vocabulary> tag. This dictionary contains specific terms and their preferred translations for the target domain, structured as `[SOURCE_TERM]=[TARGET_TERM]` pairs on separate lines. For terms that should not be translated, simply list them as `[TERM]=[TERM]`.

<custom_vocabulary>
[CUSTOM_VOCABULARY]
</custom_vocabulary>

---

### Step-by-Step Instructions (Internal Process)

**Step 1: Understand Input Type**

Internally acknowledge that the input is always raw code cell content. (This is an internal step; do not output this acknowledgement.)

---

**Step 2: Detect Source Language**

Internally identify the language of the natural language content (e.g., French, English) found **ONLY within comments and string literals**. This is the language you will be translating FROM.
If the document's natural language (in comments/strings) is already in **English**, no translation is needed. (This is an internal step; do not output this detection.)

---

**Step 2.5: Apply Custom Vocabulary (if provided)**

If a `<custom_vocabulary>` is present in the input, internally load these terms and their translations. When performing translation in Step 4, you **MUST PRIORITIZE** these custom translations for the exact terms or phrases specified in the dictionary, **BUT ONLY when they appear as natural language within comments or string literals**. **ABSOLUTELY DO NOT** apply dictionary translations to code keywords, variable names, function names, or any other elements identified as non-translatable code syntax. This dictionary serves as a **HIGH-PRIORITY OVERRIDE** for specific natural language terms and phrases within the designated translatable areas.

---

**Step 3: CRITICAL - Identify and PRESERVE Non-Translatable Elements (ALL Code Syntax and Identifiers)**

It is **ABSOLUTELY CRITICAL** that you **DO NOT TRANSLATE OR ALTER ANY** of the following code elements or syntax. These **MUST** be preserved **EXACTLY AS-IS**:
*   **ALL code syntax:** This includes but is not limited to keywords (e.g., `if`, `for`, `def`, `class`, `import`), operators (e.g., `+`, `=`, `==`), delimiters (e.g., `(`, `)`, `[`, `]`, `{`, `}`), colons (`:`), semicolons (`;`).
*   **ALL Identifiers:** This includes **ALL** variable names, function names, class names, module names, argument names (e.g., `my_variable`, `calculateSum`, `MyClass`). **DO NOT TRANSLATE THESE.**
*   **Numerical literals:** (e.g., `10`, `3.14`, `0xAF`).
*   **Boolean literals:** (e.g., `True`, `False`, `null`, `None`).
*   **Entire code structure:** This includes control flow structures, function definitions, class definitions, import statements, etc.
*   **Indentation and ALL white space:** Preserve all original spacing and line breaks.
*   **File paths, URLs, and any identifiers that are NOT part of natural language within comments or string literals.**

**To reiterate: If it is part of the code's logic, structure, or naming, IT MUST NOT BE TRANSLATED.**

---

**Step 4: Meticulously Translate ALL Source Natural Language (ONLY within Comments and String Literals)**

Once the source language is identified (Step 2), and after prioritizing any custom vocabulary (Step 2.5), you **MUST** translate **EVERY INSTANCE** of natural language from that source language into **English**. This translation applies **EXCLUSIVELY** to natural language found within:

1.  **Comments:** This includes single-line comments (e.g., `# This is a comment` in Python, `// This is a comment` in JavaScript/Java/C++) and multi-line/block comments (e.g., `"""Docstring comments"""` in Python, `/* block comments */` in JavaScript/Java/C++).
2.  **String literals:** This includes all text enclosed in quotes (e.g., `"Hello world"`, `'Error message'`, ``` `Template literal with ${expressions}` ``` - *translate the natural language parts of template literals, leaving expressions like `${expressions}` untouched*).

**EVERY piece of source natural language within these specific contexts (comments and string literals) MUST be translated.** This includes, but is not limited to:
*   Descriptive sentences and paragraphs within comments or multi-line strings (docstrings).
*   Short phrases, single words, and common connecting words (e.g., 'and', 'or', 'if', 'else', 'try', 'catch', 'finally' *when these words are used as part of natural language within a comment or string*, not as code keywords themselves).
*   Captions and inline explanations within strings.

    *   Example (Python, Source: English, Target: Ukrainian):
        ```python
        # This function calculates the sum of two numbers.
        def add_numbers(a, b):
            """
            Returns the sum of a and b.
            Also, it handles a special case for x.
            """
            print("Calculation started for 'process_data'") # Log message, 'process_data' is an identifier
            # Another note: check the input carefully.
            x = "This is a final string message before returning."
            return a + b
        ```
        ->
        ```python
        # Ця функція обчислює суму двох чисел.
        def add_numbers(a, b):
            """
            Повертає суму a та b.
            Також, вона обробляє особливий випадок для x.
            """
            print("Обчислення розпочато для 'process_data'") # Повідомлення журналу, 'process_data' є ідентифікатором
            # Ще одна примітка: уважно перевірте вхідні дані.
            x = "Це кінцеве рядкове повідомлення перед поверненням."
            return a + b
        ```
    *   Example (Python, Source: French, Target: English - showing preservation of variable names):
        ```python
        # La variable 'nom_utilisateur' est une chaîne de caractères.
        nom_utilisateur = "Bonjour le monde" # Un message de salutation
        # Vérifier si c'est vide.
        ```
        ->
        ```python
        # The variable 'nom_utilisateur' is a string.
        nom_utilisateur = "Hello world" # A greeting message
        # Check if it's empty.
        ```

**Preservation of Original Formatting and Syntax within Translatable Segments:**
During translation of natural language within comments and strings:
*   **DO NOT** escape, fix, or reformat anything. Preserve the original character encoding (assume UTF-8).
*   Keep **line breaks** within multi-line comments or strings exactly as they are.
*   Keep **spacing** within comments and strings as it is, unless linguistic changes in **English** naturally alter word spacing.
*   Handle **partial or malformed syntax** (e.g., unclosed strings or comments): Translate identifiable natural language within them if possible, but **DO NOT ATTEMPT TO FIX THE SYNTAX**. Preserve the malformed syntax exactly as it is.

**Handling Ambiguity within Natural Language Segments:**
If, after identifying a segment as source language natural language (within a comment or string), you are unsure about a specific word *within that segment*:
*   First, check the custom vocabulary.
*   Then, attempt translation based on the surrounding context.
*   **The directive to translate ALL identified source natural language within these contexts takes PRECEDENCE.**
*   Only if a word *within an already identified natural language phrase* strongly appears to be an untranslatable proper noun, a highly specific technical term with no direct equivalent in **English**, or an identifier accidentally caught, *and it's not part of a common connecting phrase or covered by the custom vocabulary*, can it be left untranslated. However, common connecting words (conjunctions, prepositions), verbs, adjectives, and common nouns in the source language **MUST ALWAYS be translated.** Be meticulous.

---

## Output Format Requirements (**STRICT**)

*   You **MUST** return **ONLY** the translated content. This content **MUST** be wrapped inside **A SINGLE, ALL-ENCOMPASSING TAG**: `<output> ... </output>`.
*   **This single `<output>` tag MUST wrap the ENTIRE processed version of the original document content from the first character to the last.**
*   **ABSOLUTELY NO TEXT OR EXPLANATION BEFORE THE `<output>` TAG OR AFTER THE `</output>` TAG.**
*   **DO NOT** wrap the output in triple backticks (```) or add any language tags like `markdown`, `text`, `python`, etc., either inside or outside the `<output>` tags.
*   The output content inside `<output> ... </output>` must be **RAW**, **line-accurate** (preserving all original line breaks), and aim to be **byte-faithful** for non-translated parts.
*   **DO NOT output ANY of your internal analysis, reasoning, detected language, document type, or any other conversational text or apologies.** Your response should be SOLELY the `<output>` tag and its content.

---

## Absolute Do-Nots (Summary of Critical Restrictions)

*   **DO NOT** correct or alter any broken or unclosed code syntax. Preserve it.
*   **DO NOT** add any formatting or beautification to the code.
*   **DO NOT** escape special characters if they were not escaped in the input. Preserve them.
*   **DO NOT** add any extra comments, ellipses, or summaries *within the code* unless they are direct translations of existing source language comments.
*   **ABSOLUTELY DO NOT** include any text, explanation, apologies, or any other content outside the single, all-encompassing `<output> ... </output>` tags.
*   **DO NOT** prematurely close the `<output>` tag. It must encompass the entire processed document.
*   **DO NOT TRANSLATE CODE IDENTIFIERS** (variable names, function names, class names, etc.). This is a CRITICAL rule.
*   **DO NOT ALTER CODE STRUCTURE OR SYNTAX.** This is also a CRITICAL rule.

---

## Special Cases

*   **If All Translatable Content is Already in English:** If your internal analysis (Step 2) determines that all natural language content *within comments and string literals* is ALREADY in **English**, then you **MUST** return the entire original document **UNCHANGED** within the `<output>` tags. No translation actions should be performed.
*   **Empty Input Document:** If the provided `<document>` tag is empty (e.g., `<document></document>`), you **MUST** return an empty document within the output tags: `<output></output>`.

---

### Begin Translation Process

Internally, before generating any output, you **MUST** meticulously follow these preparation steps:
1.  **Analyze the Input Document:**
    *   Acknowledge internally that the input is raw code cell content.
    *   Determine the source language of the natural language content found **EXCLUSIVELY within comments and string literals**.
    *   If a `<custom_vocabulary>` tag is present, load and prepare the custom vocabulary entries. Remember, these are **HIGH-PRIORITY** overrides for natural language translation within comments/strings only.
2.  **Internal Confirmation of Critical Rules (Mental Checklist - DO NOT OUTPUT THIS):**
    *   **Confirm understanding:** What parts **MUST NOT** be translated? (Answer: ALL code, ALL identifiers, ALL structure, numerical/boolean literals, etc. Only natural language in comments/strings is translated).
    *   **Confirm understanding:** What parts **MUST** be translated? (Answer: ALL source natural language **EXCLUSIVELY** within comments and string literals, prioritizing custom vocabulary).
    *   **Confirm understanding:** What is the **EXACT** output format? (Answer: The entire processed document, wrapped in a single `<output>...</output>` tag, with NO other text or formatting).
    Your internal goal is a 100% accurate translation of all identified source natural language within the specified scope (comments and string literals), while leaving ALL other content 100% identical to the input.

Once these internal checks are complete, proceed to:
3.  **Perform the Translation:** Translate **ALL** identified source natural language segments (within comments and string literals ONLY) into **English**, adhering strictly to all rules specified (especially preservation of code, structure, identifiers, and use of custom vocabulary). Process the **ENTIRE input document from start to finish.**
4.  **Return the Result:** Generate the final output, which consists **ONLY** of the processed document content wrapped in a single, all-encompassing `<output>` tag:

<output>
[translated document here, ensuring all code is preserved and only NL in comments/strings is translated]
</output>

**Nothing else. No explanations. No apologies. No extra text.**
````

## Conclusions
In most cases, the `llama` model seems to be overwhelmed with the prompt and
amount of details to handle. The first suggestion is that `llama`'s **real**
context window is too small for the prompt. The evidence for such suggestion is
that the model has been asked why it had translated what it must not translate 
and what instructions have been provided, the model's response were all the
instructions listed in the prompt. However, the second output is not always
better, sometimes it's corrected, sometimes, the model doesn't follow the
instruction that the model listed itself.

## Future work
The future work is to refine again the prompt and try with the `llama` model.
Other open-source and open-weighted models should be tested using the same
tests. Such models could be:
- `gemma3-27b` model from Google
- `mixtral` models from Mistral
- `qwen` model
- other models to explore
