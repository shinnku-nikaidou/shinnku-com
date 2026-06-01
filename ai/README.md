# shinnku-com AI service

Pipeline overview:

```
distill/gal.txt          ┐
distill/corporation.txt  ┘─► gcp_distill.py (Gemini on Vertex AI)
                                ├─► distill/fine_tune_data_gal.jsonl
                                └─► distill/fine_tune_data_corporation.jsonl
                                         │
                                         ▼ distill/merge_jsonl.py
                                     datal.jsonl
                                         │
                                         ▼ bge_rag_train.py
                              database/chroma_aisearch  (chunked, /findname)
                              database/chroma_intro     (full doc, /intro)
                                         │
                                         ▼ serve.py (Falcon ASGI)
                                  /intro     /findname
```

## Setup

```
uv sync           # or: pip install -e .
```

## 1. Distill introductions with Gemini (optional, slow & costs $)

```
export GCP_KEY_PATH=/abs/path/to/service-account.json
export GCP_PROJECT_ID=your-project-id
python -m ai.gcp_distill           # appends to distill/*.jsonl, resumable
```

Seed lists live in `distill/gal.txt` and `distill/corporation.txt`.

## 2. Merge into the training jsonl

```
python ai/distill/merge_jsonl.py   # produces ai/datal.jsonl
```

The committed `datal.jsonl` is already the merge of the two `fine_tune_data_*`
files, so you can skip steps 1–2 unless you add new names.

## 3. Build Chroma vector stores

```
python -m ai.bge_rag_train         # ~10 min on CPU, writes ai/database/*
```

## 4. Serve

```
uvicorn ai.serve:app --port 2998
```
