# LaTeX chunking

## Introduction
`LaTeX` is one of the most popular markup languages especially in the
scientific community. Consequently, it's support is one of the most prioritized
among all the markup languages.

The library already provides the functionality of chunk translation and
retrieval. First translation examples of the jupyter notebooks showed 
very satisfying results, thus the expansion to the markup languages
including LaTeX is reasonable. The nuance is that the jupyter notebooks
is divided into cells (here we call chunks) by default. Consequently, 
the document division into chunks is the only missing functionality 
for workable translation. 

## Tool choice
There are two libraries that work well with parsing `LaTeX` documents into an *Abstract Syntax Tree*:
- `pandoc`
- `pylatexenc`

`pandoc` is a well-known tool that is common to use and work not only with
`LaTeX`, but also with `markdown` and other markup languages. The principal drawback
that forced us to `pylatexenc` is the absence of possibility to extract back
the document source code of each particular node, unlike `pylatexenc` that 
provides such functionality.

`pylatexenc` is a lightweight library to convert `latex` to unicode and vice versa. 
We are concentrating on it's `latexwalker` module that parses `latex` code into abstract nodes
and understands most of the `latex` commands and macros.

## Implementation
Firstly, we verify if the provided document contains `document` environment.
1. If it contains, then we parse the preamble (all the contents before the
   `document` environment begins) and then we parse the contents inside of the 
   `document` environment as it describe in the (2).
2. If the document doesn't contain such environment, then it's separated in
   such chunks:  
   - environment
   - contents between two environments

The preamble as well as the contents after the document environment are divided
into chunks by commands/macros.

## Metadata
The metadata of each chunk is essential functionality of the library and the
translation project that the library can't work properly without.

The chunks are stored as comments before each chunk in the next manner:
```tex
% --- CHUNK_METADATA_START ---
% key: value 
% --- CHUNK_METADATA_END ---
```

Example:
```tex
% --- CHUNK_METADATA_START ---
% needs_review: True
% src_checksum: 57aca947136707d25d0ced893989d252e214669249f34dd0165e9fe7bb00f03b
% --- CHUNK_METADATA_END ---
```

## Faced problems and future work
The problem that is faced that very rarely chunks can be bigger that we would expect
and would like to make them smaller in order to keep the number of tokens
passed to the input of the model as optimal as possible. As a consequence, 
the rework of the chunking process is required. Possible ways would be to 
count the number of tokens in the chunk and if it exceeds the limit, then 
separate the same chunk on more chunks following the strategy above.

## Conclusion
The main goal of this report has been to introduce the chunking process of the
LaTeX documents in order to improve the translation quality.
