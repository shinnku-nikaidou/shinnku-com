from langchain_huggingface import HuggingFaceEmbeddings
from langchain_chroma import Chroma

emb = HuggingFaceEmbeddings(model_name="BAAI/bge-large-zh-v1.5")

vectorstore_intro = Chroma(
    persist_directory="./database/chroma_intro", embedding_function=emb
)
retriever_intro = vectorstore_intro.as_retriever()

vectorstore_findname = Chroma(
    persist_directory="./database/chroma_aisearch", embedding_function=emb
)

retriever_findname = vectorstore_findname.as_retriever()
