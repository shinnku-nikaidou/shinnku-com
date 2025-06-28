from langchain_community.document_loaders import JSONLoader
from langchain.text_splitter import RecursiveCharacterTextSplitter
from langchain.embeddings import HuggingFaceEmbeddings

# it can't be compiled due to torch2 is somewhat unsound
# from langchain_huggingface import HuggingFaceEmbeddings
from langchain_chroma import Chroma
from langchain_ollama import OllamaLLM
from langchain.chains import RetrievalQA
import re


def metadata_func(record: dict, metadata: dict) -> dict:
    metadata["prompt"] = record.get("prompt")
    return metadata


loader = JSONLoader(
    file_path="datal.jsonl",
    jq_schema=".",
    content_key="completion",
    metadata_func=metadata_func,
    text_content=True,
    json_lines=True,
)

docs = loader.load()

for doc in docs:
    doc.page_content = doc.metadata["prompt"] + "\n" + doc.page_content

splitter = RecursiveCharacterTextSplitter(chunk_size=512, chunk_overlap=100)
chunks = splitter.split_documents(docs)

emb = HuggingFaceEmbeddings(model_name="BAAI/bge-large-zh-v1.5")

vectorstore = Chroma.from_documents(chunks, emb, persist_directory="./beg_rag_chroma")

vectorstore_generate = Chroma.from_documents(
    docs, emb, persist_directory="./beg_rag_chroma_generate"
)

vectorstore = Chroma(persist_directory="./beg_rag_chroma", embedding_function=emb)

llm = OllamaLLM(model="qwen3:8b")

qa = RetrievalQA.from_chain_type(
    llm=llm, chain_type="stuff", retriever=vectorstore.as_retriever()
)


def split_qwen3(answer) -> str:
    cleaned_result = re.sub(
        r"<think>.*?</think>\s*", "", answer, flags=re.DOTALL
    ).strip()
    return cleaned_result


split_qwen3(qa.invoke("秽翼的尤斯蒂娅"))
split_qwen3(qa.invoke("千恋万花"))
split_qwen3(qa.invoke("寒蝉鸣泣之时"))
split_qwen3(qa.invoke("美少女万華鏡"))
split_qwen3(qa.invoke("星空的记忆"))
split_qwen3(qa.invoke("详细介绍夏空彼方"))


retriever = vectorstore.as_retriever()
