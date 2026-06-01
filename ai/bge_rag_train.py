"""Build the two Chroma vector stores consumed by ``ai/serve.py``.

Inputs:
    datal.jsonl  -- merged distilled dataset, one record per line:
                    {"prompt": "<game/company name>", "completion": "<intro>"}

Outputs (written next to this script):
    database/chroma_aisearch  -- chunked store, used by /findname (AI search)
    database/chroma_intro     -- full-doc store, used by /intro (intro card)

Why two stores from the same datal.jsonl?
    The two endpoints have opposite retrieval needs:
      * /findname (AI search) takes a fuzzy user query and must return the
        canonical game/company *name*. Smaller chunks give finer-grained
        similarity matches; we then read ``metadata.prompt`` (the name).
      * /intro takes an exact name and must return the *whole* introduction
        text. Splitting would leave only a fragment in ``page_content``, so
        we index each record as a single un-split document.
    Same source data, same embedding model -- different granularity, so
    two separate Chroma collections.
"""

from pathlib import Path

from langchain_chroma import Chroma
from langchain_community.document_loaders import JSONLoader
from langchain_huggingface import HuggingFaceEmbeddings
from langchain_text_splitters import RecursiveCharacterTextSplitter

HERE = Path(__file__).resolve().parent
DATA_PATH = HERE / "datal.jsonl"
DB_DIR = HERE / "database"
AISEARCH_DB = DB_DIR / "chroma_aisearch"  # chunked, for /findname
INTRO_DB = DB_DIR / "chroma_intro"  # full docs, for /intro


def metadata_func(record: dict, metadata: dict) -> dict:
    metadata["prompt"] = record.get("prompt")
    return metadata


def build() -> None:
    loader = JSONLoader(
        file_path=str(DATA_PATH),
        jq_schema=".",
        content_key="completion",
        metadata_func=metadata_func,
        text_content=True,
        json_lines=True,
    )
    docs = loader.load()

    for doc in docs:
        doc.page_content = f"{doc.metadata['prompt']}\n{doc.page_content}"

    splitter = RecursiveCharacterTextSplitter(chunk_size=512, chunk_overlap=100)
    chunks = splitter.split_documents(docs)

    emb = HuggingFaceEmbeddings(model_name="BAAI/bge-large-zh-v1.5")

    DB_DIR.mkdir(parents=True, exist_ok=True)

    print(f"[bge_rag_train] writing {len(chunks)} chunks -> {AISEARCH_DB}")
    Chroma.from_documents(chunks, emb, persist_directory=str(AISEARCH_DB))

    print(f"[bge_rag_train] writing {len(docs)} full docs -> {INTRO_DB}")
    Chroma.from_documents(docs, emb, persist_directory=str(INTRO_DB))

    print("[bge_rag_train] done.")


if __name__ == "__main__":
    build()
