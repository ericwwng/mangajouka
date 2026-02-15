import requests
import dotenv

from typing import Optional
from langchain_openai import ChatOpenAI
from engine import MangaRecommendationEngine
from flask import Flask, request, jsonify


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


async def run_workflow(batch_inputs) -> list[str]:
    llm = load_llm()

    if not llm:
        print("LLM not initialized. Exiting main")
        return []

    # Instantiate the engine
    engine = MangaRecommendationEngine(llm)
    full_chain = engine.build_full_chain()

    # Execution
    try:
        response = await full_chain.abatch(batch_inputs)
        # response = await full_chain.ainvoke(
        #    {"manga_name": "One Piece", "manga_information": manga_info}
        # )

        return response
    except Exception as e:
        print(f"Error: {e}")

    return []


app = Flask(__name__)

dotenv.load_dotenv()


@app.post("/recommend/<user_id>")
async def get_bulk_recommendations(user_id):
    # 1. Get the list of manga names from the request body
    # Expected JSON: {"manga_names": ["Naruto", "Berserk", "Monster"]}
    data = request.get_json()
    manga_names = data.get("manga_names", [])

    if not manga_names:
        return jsonify({"error": "No manga names provided"}), 400

    try:
        # 2. Fetch user preferences once
        # manga_info = call_get_manga_information_api("c5f647ea-15a2-4977-9981-09395ee06761")
        user_prefs = call_get_manga_information_api(user_id)

        # 3. Prepare inputs for batch processing
        # We create a list of dicts, one for each manga
        batch_inputs = [
            {"manga_name": name, "manga_information": user_prefs}
            for name in manga_names
        ]

        # 4. Execute all recommendations in parallel
        results = await run_workflow(batch_inputs)

        # 5. Pair names with their specific results
        response_data = dict(zip(manga_names, results))

        return jsonify({"user_id": user_id, "recommendations": response_data}), 200

    except Exception as e:
        return jsonify({"error": str(e)}), 500


# if __name__ == "__main__":
#     dotenv.load_dotenv()
#     asyncio.run(run_workflow())
