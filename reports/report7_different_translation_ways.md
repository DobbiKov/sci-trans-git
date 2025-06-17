# Trying different translation ways with small models

It is currently a draft.

There are several ideas of how to pass data to LLM:
1. <TEXT> tag for each text part and <PH> for untranslatable parts, example:
```xml
<TEXT id="2">Soit </TEXT><PH id="3" original="$A \subset E$"/><TEXT id="4"> borné si </TEXT><PH id="5" original="$\exists R > 0$"/><TEXT id="6"> tel que </TEXT><PH id="7" original="$A \subset B(x, R)$"/><TEXT id="8">.</TEXT>
```

Advantages: math and text never alter, we keep the structure and syntax
Cons: LLM doesn't change the order of words if it is needed, especially if one word is emphasized in using latex commands. For example: in French an adjective goes after a noun, in English is reversed.

2. Only one text tag, everything is translatable except <PH> tags that contain id and original source for context, example:
```xml
<TEXT id="2">Soit <PH id="3" original="$A \subset E$"/> borné si <PH id="5" original="$\exists R > 0$"/> tel que <PH id="7" original="$A \subset B(x, R)$"/>.</TEXT>
```

Advantages: LLM has more possibilities to alter the text
Cons: smaller models can put more text that has been outside of math environment inside of it

---
There were also idea to provide firstly the source untagged code for the LLM so
that it understands the context and then the xml tagged version but small
models tend to be overwhelmed due to the real size of the context windows.

## More important notes
Gemma is unusable, especially in the second case but understands better how to write in Ukrainian language.
