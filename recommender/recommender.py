import requests
import dotenv
import asyncio

from typing import Optional
from langchain_openai import ChatOpenAI
from engine import MangaRecommendationEngine


def call_get_manga_information_api(user_id):
    url = "http://localhost:8000/api/manga_information"
    query_params = {"user_id": user_id}

    try:
        response = requests.get(url, params=query_params)

        # Throws exception if status is 4xx or 5xx
        response.raise_for_status()

        return response.json()
    except requests.exceptions.RequestException as e:
        print(f"Error fetching manga info for user: {user_id} {e}")


def load_llm() -> Optional[ChatOpenAI]:
    try:
        llm: Optional[ChatOpenAI] = ChatOpenAI(model="gpt-4o-mini", temperature=0.7)
        if llm:
            print(f"Language model initialized: {llm.model_name}")
    except Exception as e:
        print(f"Error initializing language model: {e}")
        llm = None

    return llm


async def run_workflow():
    llm = load_llm()

    if not llm:
        print("LLM not initialized. Exiting main")
        return

    # Instantiate the engine
    engine = MangaRecommendationEngine(llm)
    full_chain = engine.build_full_chain()

    # Data Fetching
    manga_info = call_get_manga_information_api("c5f647ea-15a2-4977-9981-09395ee06761")

    # Execution
    try:
        response = await full_chain.ainvoke(
            {"manga_name": "One Piece", "manga_information": manga_info}
        )
        print(response)
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    dotenv.load_dotenv()
    asyncio.run(run_workflow())
