"""Concat distilled JSONL files into ``ai/datal.jsonl`` for bge_rag_train.

Dedupes by ``prompt`` (last write wins).
"""

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
AI_DIR = HERE.parent
SOURCES = [
    HERE / "fine_tune_data_gal.jsonl",
    HERE / "fine_tune_data_corporation.jsonl",
]
DEST = AI_DIR / "datal.jsonl"


def main() -> None:
    merged: dict[str, str] = {}
    for src in SOURCES:
        if not src.exists():
            print(f"[merge_jsonl] skip missing {src}")
            continue
        with open(src, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                rec = json.loads(line)
                merged[rec["prompt"]] = rec["completion"]

    with open(DEST, "w", encoding="utf-8") as f:
        for prompt, completion in merged.items():
            f.write(
                json.dumps(
                    {"prompt": prompt, "completion": completion},
                    ensure_ascii=False,
                )
                + "\n"
            )
    print(f"[merge_jsonl] wrote {len(merged)} records -> {DEST}")


if __name__ == "__main__":
    main()
