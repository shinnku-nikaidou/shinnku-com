import json
from typing import Callable
from google import genai
from google.genai import types

from google.oauth2 import service_account

KEY_PATH = "yourproject-id-xxxxxxx.json"
PROJECT_ID = "yourproject-id"
LOCATION = "global"

credentials = service_account.Credentials.from_service_account_file(
    KEY_PATH, scopes=["https://www.googleapis.com/auth/cloud-platform"]
)

client = genai.Client(
    vertexai=True, project=PROJECT_ID, location=LOCATION, credentials=credentials
)


def generate(text: str, client: genai.Client) -> str:
    print(f"Generating content for: {text}")
    model = "gemini-2.5-pro-preview-05-06"
    contents = [
        types.Content(
            role="user",
            parts=[types.Part.from_text(text=text)],
        ),
    ]
    tools = [
        types.Tool(google_search=types.GoogleSearch()),
    ]
    generate_content_config = types.GenerateContentConfig(
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
        tools=tools,
    )
    full_text = ""
    for chunk in client.models.generate_content_stream(
        model=model,
        contents=contents,
        config=generate_content_config,
    ):
        if (
            not chunk.candidates
            or not chunk.candidates[0].content
            or not chunk.candidates[0].content.parts
        ):
            continue
        print(chunk.text, end="")
        full_text += chunk.text or ""
    print("\n")
    return full_text


def intro_galgame(game_name: str) -> str:
    prompt = f"请用中文介绍一下galgame游戏：{game_name}"
    return prompt


def intro_corporation(company_name: str) -> str:
    prompt = f"请用中文介绍一下这家游戏会社：{company_name}"
    return prompt


generate(intro_galgame("GINKA"), client)
generate(intro_galgame("D.C.IIP.C.～ダ・カーポII～プラスコミュニケーション"), client)
generate(intro_corporation("ゆずソフト"), client)


def export_to_jsonl(
    inputs: list[str],
    transmute: Callable[[str], str],
    output_path: str = "output.jsonl",
):
    """
    make a jsonl file for fine-tune
    :param inputs: list of input strings
    :param output_path: path to save the jsonl file
    """
    for prompt in inputs:
        completion = generate(transmute(prompt), client)
        if completion == "":
            print(f"Error: No completion generated for prompt: {prompt}")
            continue
        record = {
            "prompt": prompt,
            "completion": completion,
        }
        with open(output_path, "a", encoding="utf-8") as fout:
            fout.write(json.dumps(record, ensure_ascii=False) + "\n")
        print(f"Generated completion for: {prompt}")
        print("\n\n")


def read_file(file_path):
    """
    Read a file and return its content as a list of strings.
    :param file_path: Path to the input file.
    :return: List of strings, each representing a line in the file.
    """
    with open(file_path, "r", encoding="utf-8") as f:
        lines = f.readlines()
    return [line.strip() for line in lines]


if __name__ == "__main__":
    gal_inputs = read_file("galgames.txt")
    export_to_jsonl(gal_inputs, intro_galgame, "fine_tune_data.jsonl")
