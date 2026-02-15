import requests
import dotenv
import asyncio
from typing import Optional

from langchain_openai import ChatOpenAI
from langchain_core.prompts import ChatPromptTemplate
from langchain_core.output_parsers import StrOutputParser
from langchain_core.runnables import Runnable, RunnableParallel, RunnablePassthrough


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

    fetch_manga_description_chain: Runnable = (
        ChatPromptTemplate.from_messages(
            [
                ("system", "Find a description for the manga with this name"),
                ("user", "{manga_name}"),
            ]
        )
        | llm
        | StrOutputParser()
    )

    fetch_manga_reviews_chain: Runnable = (
        ChatPromptTemplate.from_messages(
            [
                ("system", "Find reviews for the manga with this name"),
                ("user", "{manga_name}"),
            ]
        )
        | llm
        | StrOutputParser()
    )

    map_chain = RunnableParallel(
        {
            "description": fetch_manga_description_chain,
            "reviews": fetch_manga_reviews_chain,
            "manga_name": RunnablePassthrough(),
            "manga_information": RunnablePassthrough(),
        }
    )

    synthesis_prompt = ChatPromptTemplate.from_messages(
        [
            (
                "system",
                """Based on the following information of the given manga:
        Name: {manga_name}
        Description: {description}
        Reviews: {reviews}
        Favorite User Manga Information: {manga_information}
        Provide a recommendation rating for the given manga based on the user's favorite manga information from a scale of 0.0/10.0,
        and describe how the recommendation relates to the user's preferences""",
            ),
            ("user", "Given manga_name: {manga_name}"),
        ]
    )

    full_chain = map_chain | synthesis_prompt | llm | StrOutputParser()

    manga_information = call_get_manga_information_api(
        "c5f647ea-15a2-4977-9981-09395ee06761"
    )

    try:
        response = await full_chain.ainvoke(
            {"manga_name": "Kiss x Sis", "manga_information": manga_information}
        )
        print(response)
    except Exception as e:
        print(f"\n An error occurred during agent execution: {e}")


if __name__ == "__main__":
    dotenv.load_dotenv()
    asyncio.run(run_workflow())
