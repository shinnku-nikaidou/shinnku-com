"""Distill galgame / company introductions with Gemini on Vertex AI.

Reads seed name lists from ``distill/gal.txt`` and ``distill/corporation.txt``
and appends one ``{"prompt", "completion"}`` JSONL record per name to
``distill/fine_tune_data_gal.jsonl`` / ``distill/fine_tune_data_corporation.jsonl``.

Requires a Vertex AI service-account key file; set ``GCP_KEY_PATH`` and
``GCP_PROJECT_ID`` env vars, or edit the constants below.
"""

import json
import os
from pathlib import Path
from typing import Callable

from google import genai
from google.genai import types
from google.oauth2 import service_account

HERE = Path(__file__).resolve().parent
DISTILL_DIR = HERE / "distill"

KEY_PATH = os.environ.get("GCP_KEY_PATH", "service-account.json")
PROJECT_ID = os.environ.get("GCP_PROJECT_ID", "your-project-id")
LOCATION = os.environ.get("GCP_LOCATION", "global")
MODEL = os.environ.get("GCP_MODEL", "gemini-2.5-pro-preview-05-06")


def make_client() -> genai.Client:
    credentials = service_account.Credentials.from_service_account_file(
        KEY_PATH, scopes=["https://www.googleapis.com/auth/cloud-platform"]
    )
    return genai.Client(
        vertexai=True,
        project=PROJECT_ID,
        location=LOCATION,
        credentials=credentials,
    )


def generate(text: str, client: genai.Client) -> str:
    print(f"Generating content for: {text}")
    contents = [
        types.Content(role="user", parts=[types.Part.from_text(text=text)]),
    ]
    config = types.GenerateContentConfig(
        temperature=1,
        top_p=1,
        seed=0,
        max_output_tokens=65535,
        safety_settings=[
            types.SafetySetting(
                category=types.HarmCategory.HARM_CATEGORY_HATE_SPEECH,
                threshold=types.HarmBlockThreshold.BLOCK_NONE,
            ),
            types.SafetySetting(
                category=types.HarmCategory.HARM_CATEGORY_DANGEROUS_CONTENT,
                threshold=types.HarmBlockThreshold.BLOCK_NONE,
            ),
            types.SafetySetting(
                category=types.HarmCategory.HARM_CATEGORY_SEXUALLY_EXPLICIT,
                threshold=types.HarmBlockThreshold.BLOCK_NONE,
            ),
            types.SafetySetting(
                category=types.HarmCategory.HARM_CATEGORY_HARASSMENT,
                threshold=types.HarmBlockThreshold.BLOCK_NONE,
            ),
        ],
        tools=[types.Tool(google_search=types.GoogleSearch())],
    )

    full_text = ""
    for chunk in client.models.generate_content_stream(
        model=MODEL, contents=contents, config=config
    ):
        if (
            not chunk.candidates
            or not chunk.candidates[0].content
            or not chunk.candidates[0].content.parts
        ):
            continue
        print(chunk.text, end="")
        full_text += chunk.text or ""
    print()
    return full_text


def intro_galgame(game_name: str) -> str:
    return f"请用中文介绍一下galgame游戏：{game_name}"


def intro_corporation(company_name: str) -> str:
    return f"请用中文介绍一下这家游戏会社：{company_name}"


def read_lines(file_path: Path) -> list[str]:
    with open(file_path, "r", encoding="utf-8") as f:
        return [line.strip() for line in f if line.strip()]


def export_to_jsonl(
    inputs: list[str],
    transmute: Callable[[str], str],
    output_path: Path,
    client: genai.Client,
) -> None:
    # Skip prompts that already have a completion to make the run resumable.
    done: set[str] = set()
    if output_path.exists():
        with open(output_path, "r", encoding="utf-8") as fin:
            for line in fin:
                try:
                    done.add(json.loads(line)["prompt"])
                except Exception:
                    continue

    with open(output_path, "a", encoding="utf-8") as fout:
        for prompt in inputs:
            if prompt in done:
                continue
            completion = generate(transmute(prompt), client)
            if not completion:
                print(f"Error: empty completion for: {prompt}")
                continue
            fout.write(
                json.dumps(
                    {"prompt": prompt, "completion": completion},
                    ensure_ascii=False,
                )
                + "\n"
            )
            fout.flush()


if __name__ == "__main__":
    client = make_client()
    export_to_jsonl(
        read_lines(DISTILL_DIR / "gal.txt"),
        intro_galgame,
        DISTILL_DIR / "fine_tune_data_gal.jsonl",
        client,
    )
    export_to_jsonl(
        read_lines(DISTILL_DIR / "corporation.txt"),
        intro_corporation,
        DISTILL_DIR / "fine_tune_data_corporation.jsonl",
        client,
    )
    print("done.")
