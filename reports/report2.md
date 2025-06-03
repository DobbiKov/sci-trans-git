# Report 2 - 3.06.2025

## Lib update
This week mostly has been spent on implementing the translation functionalities
for jupyter notebooks as the `jupytext` library provides an API to work with
the notebooks, extract cells' contents and add required metadata. 

### Translation
For the translation, the `gemini-2.0-flash` model is still used, upon of it,
there is a translator module that simplifies the change of model in the future.
For the chunking process the `jupytext` python module is used.

### Translation DataBase
In order to not translate already translated chunks the translation database
feature has been implemented. The idea is to store the original text with its
checksums as well as the translations with the checksums and to maintain 
a database with checksum correspondence (i.e which original chunk corresponds
to which translation).

For such feature a new module named `trans_db` and `translation_retrieval` have been added. 
- `trans_db` implements all the _CRUD_ for the checksums, contents and the correspondence
- `translation_retrieval` is the middleware between the `trans_db`,
  `translator` and the `project_manager`. It takes the contents to translate
  and verifies that it is not translated.
  - If it is translated, it returns the translation from the database
  - If it is not, then it calls the appropriate function to translate

A translation project structure looks in the next way:
```
project_root/
├── trans_conf.json
├── source_dir/
│   └── …
├── translate_temp_fr/
│   └── …
└── trans_git_db/
    ├── English/
    │   ├── <checksum1_en>
    │   └── <checksum2_en>
    ├── French/
    │   ├── <checksum1_fr>
    │   └── <checksum2_fr>
    └── correspondence_db.csv
```

and the `correspondence_db.csv` file looks like:
```
English,French
<checksum1_en>, <checksum1_fr>
<checksum2_en>, <checksum2_fr>
```

The files names by `<checksum*_**>` store the content itself (i.e the original
or translated text that checksum has been calculated of)

Easy to guess that `<checksum1_en>` corresponds to the `<checksum1_fr>`, then
if we encounter a chunk in the original document and the checksum of that chunk
is `<checksum1_en>`, then there is no need to translate that chunk, we can extract 
the translation from the file named `<checksum1_fr>`.


### Translation correction
The machine translation, unfortunately, is not always as accurate as we would
like it to be and it needs to be corrected by human. Consequently, the
`correction` feature has been implemented. 

The translation works in a way that after the document has been translated, 
the metadata stores the original checksum, i.e the checksum of the source chunk
that the translation has been taken of. Then, when a human proofreads the machine 
translation and edits it, using the `correct` command, the _refined_ translation is
stored in the database and will not be changed in the future translations. 

Technically, when a user uses `correct` command, the library calculates the checksum of
the (translated) chunk, searches for the original `checksum` stored in the metadata
and compares the correspondent checksum stored in the database and the one calculated
from the chunk. If it differs, the we overwrite using the new checksum.

## German translation
This week's goal has been the translation of the python course into German. The library
did its job pretty well except some issues.

Sometimes the _LLM_ sees `###` as a comment, e.g `### BEGIN SOLUTION` and translates it,
but it happens at most for 2 cells among 72 notebooks. The same issue
encountered with one word code cells such as 
```py
animaux
```
Trivially, such cell is used to see what is stored in the variable named `animaux`, 
but the _LLM_ misunderstands it as a non-code word and translates it.

Consequently, the work on prompt refinement has to be done in order to avoid such 
problems.

## Conclusion
This week, the translation database feature has been implemented as well as
translation correction and translation on non-translated chunks. However, these
feature work only for jupyter notebooks for now.

The python course has been translated into German encountering some issues.

## Future improvement
The future improvements are implementing the proper chunking for `tex` files
and other non-jupyter file types and refining prompt to avoid translation
issues.
