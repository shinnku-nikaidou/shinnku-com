import falcon
import falcon.asgi
from langchain_huggingface import HuggingFaceEmbeddings
from langchain_chroma import Chroma

emb = HuggingFaceEmbeddings(model_name="BAAI/bge-large-zh-v1.5")

vectorstore_intro = Chroma(
    persist_directory="./database/beg_rag_chroma_generate",
    embedding_function=emb,
)
retriever_intro = vectorstore_intro.as_retriever()

vectorstore_findname = Chroma(
    persist_directory="./database/beg_rag_chroma", embedding_function=emb
)

retriever_findname = vectorstore_findname.as_retriever()

ramdonshit = (
    "Two things awe me most, the starry sky "
    "above me and the moral law within me."
    "\n"
    "    ~ Immanuel Kant\n\n"
)


class Intro:
    async def on_get(self, req, resp):
        """Handles GET requests"""
        name = req.get_param("name")
        resp.content_type = falcon.MEDIA_JSON
        if name:
            try:
                results = await retriever_intro.ainvoke(name, k=1)
                if results:
                    resp.status = falcon.HTTP_200
                    resp.media = {
                        "name": name,
                        "content": results[0].page_content,
                        "message": "Success",
                    }
                else:
                    resp.status = falcon.HTTP_200
                    resp.media = {"message": "No results found."}
            except Exception as e:
                resp.status = falcon.HTTP_500
                resp.media = {"message": f"An error occurred: {str(e)}"}
        else:
            resp.status = falcon.HTTP_400
            resp.media = {"message": ramdonshit}


class FindName:
    async def on_get(self, req, resp):
        name = req.get_param("name")
        resp.content_type = falcon.MEDIA_JSON
        if name:
            try:
                results = await retriever_findname.ainvoke(name, k=12)
                if results:
                    resp.status = falcon.HTTP_200
                    resp.media = {
                        "ans": [result.metadata.get("prompt", "") for result in results]
                    }
                else:
                    resp.status = falcon.HTTP_404
                    resp.media = {"ans": []}
            except Exception as e:
                resp.status = falcon.HTTP_500
                resp.media = {"ans": [], "error": f"An error occurred: {str(e)}"}
        else:
            resp.status = falcon.HTTP_400
            resp.media = {"message": ramdonshit}


app = falcon.asgi.App()

intro = Intro()
find_name = FindName()

app.add_route("/intro", intro)
app.add_route("/findname", find_name)
