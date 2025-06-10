---
title: Translation Prototype Report
author: Yehor KOROTENKO
date: 2025-05-26
---

# Introduction
The goal of the prototype was to develop an application that can translate an entire chosen directory into one or more target languages. A French Python course was translated to show the effectiveness of the application.

# Application architecture
To make it easy to build tools in different formats—such as a command-line interface (CLI) or a graphical interface (GUI)—the core functionality was implemented as a **library**.  
A small demo CLI app sits on top of that library to simplify everyday use and testing.

| Component | Repository |
|-----------|------------|
| Library   | <https://github.com/DobbiKov/translate-dir-lib> |

## Library architecture
The library is split into the following modules:

* **project** – the main entry point; the only module that “talks” to external apps  
* **project_config** – handles project-level configuration (in the future it will mimic a Git-like layout)  
* **translator** – a *temporary* module that abstracts prompt/model selection  
* **helper** – utilities for working with text (chunking, extracting model output, etc.)

## Application logic
Git inspired the overall flow. A directory becomes a *translation project* after you initialize it with the provided command.

### Directory / language settings
Inside a project, you must mark the **source directory** and its language:

```text
project_root/
├── trans_conf.json
└── source_dir/        # language is recorded here
    └── …              # original files
````

You can add or remove **target languages**.
Adding a new target language creates a sibling directory named `[project_name]_[lang]`.
For example, project `translate_temp` with French as target yields `translate_temp_fr/`.

```text
project_root/
├── trans_conf.json
├── source_dir/
│   └── …
└── translate_temp_fr/
    └── …
```

### Syncing and translation

All files start out **untranslatable**. Two helpers are available:

* `make_translatable`
* `make_untranslatable`

`sync` copies every *untranslatable* file into the target-language directories so that each tree mirrors the source (handy for LaTeX or MyST builds).
*Translatable* files are **not** copied during sync.

To translate content you can:

* `translate_file <file> <lang>` – translates one file and writes it to the correct path under the target-language directory.
* `translate_all` – translates every *translatable* file by repeatedly calling `translate_file`.

# Large Language Model

For the prototype we chose **Gemini 2.0 Flash**:

* Google offers a generous free tier (≈ 1 500 requests / day).
* Extremely low cost—about \$0.10 / M input tokens and \$0.40 / M output tokens—far cheaper than GPT or Claude.
* Lower energy use than competing models.
* Simple, well-documented API.

## Energy consumption

Gemini 2.0 Flash is a low-latency, energy-efficient model. A recent comparison is shown below:

| Model name                      | Energy per request |
| --------------------------------| ------------------ |
| GPT-4o                                | 0.30 Wh            |
| **Gemini 2.0 Flash**                  | **0.022 Wh**       |
| Gemini 2.0 Flash-Lite                 | 0.016 Wh           |
| Claude Haiku 3.5                      | 0.22 Wh            |
| Claude Opus 3                         | 4.05 Wh            |
| Ollama 3.3 70B                        | 10 Wh              |
| Ollama 3.3 70B FP8 quantization       | 0.033 Wh           |
| Gemma 3 27B (Q4_K) (on Macbook Pro)   | 0.69 Wh            |

*Table 1 – Energy consumption of popular LLMs ([1], [2], [4], [5])*

From Table 1 we can see that **Gemini 2.0 Flash** delivers state-of-the-art results with the lowest energy footprint.

Important to note, that the there's no much information about the `Gemma 3 27B` energy consumption. 

# Python course translation

To evaluate the tool we translated a full Python course:

* **72 files** → roughly **132 000 tokens** of output text.
* Each file was split into chunks of *≤ 50 lines* (a placeholder algorithm that will be improved).
* Each request averaged **≈ 500 input tokens**, plus a **2 054-token prompt**.

Total:

* **≈ 264 requests**, each ≈ 2 554 input tokens
* **Input tokens**: 2 554 × 264 ≈ 675 000
* **Output tokens**: ≈ 132 000

Using Google’s pricing, the job cost **< \$0.50** ([3]).
Energy use: 264 × 0.022 Wh ≈ **6 Wh**—about the energy to charge a smartphone from 0 % to \~15–20 %.

# Encountered problems

LLMs still struggle with syntax-aware translation, especially when context is missing because of naive chunking:

* Short identifiers can be mistaken for natural words (and translated).
* If a code cell is split across chunks, the model may treat function or variable names as prose and translate them, requiring manual cleanup.

# Future work

1. **Re-design the chunking algorithm** to preserve context and avoid splitting code cells.
2. **Refine the prompt** (or add system-level instructions) to cut down on:

   * unwanted syntax/layout changes
   * identifiers misinterpreted as text

---

# References
- [1](https://www.analyticsvidhya.com/blog/2025/04/gemini-2-0-flash-vs-o4-mini/?utm_source=chatgpt.com)
- [2](https://www.linkedin.com/pulse/power-consumption-chatgpt-queries-high-previously-thought-julian-wong-ey28c)
- [3](https://ai.google.dev/gemini-api/docs/pricing)
- [4](https://llm-tracker.info/_TOORG/Power-Usage-and-Energy-Efficiency?utm_source=chatgpt.com)
- [5](https://www.reddit.com/r/LocalLLaMA/comments/1jc1nw1/actual_electricity_consumption_and_cost_to_run/?utm_source=chatgpt.com)
