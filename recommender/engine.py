from operator import itemgetter
from langchain_core.prompts import ChatPromptTemplate
from langchain_core.output_parsers import StrOutputParser
from langchain_core.runnables import Runnable, RunnableParallel


class MangaRecommendationEngine:
    def __init__(self, llm):
        self.llm = llm
        self.output_parser = StrOutputParser()

    def create_basic_chain(self, system_message: str) -> Runnable:
        """Helper to create sub-chains for descriptions and reviews."""
        prompt = ChatPromptTemplate.from_messages(
            [
                ("system", system_message),
                ("user", "{manga_name}"),
            ]
        )
        return prompt | self.llm | self.output_parser

    def build_full_chain(self) -> Runnable:
        # 1. Define the parallel gathering step
        map_chain = RunnableParallel(
            {
                "description": self.create_basic_chain(
                    "Find a description for the manga"
                ),
                "reviews": self.create_basic_chain("Find reviews for the manga"),
                "manga_name": itemgetter("manga_name"),
                "manga_information": itemgetter("manga_information"),
            }
        )

        # 2. Define the synthesis step
        synthesis_prompt = ChatPromptTemplate.from_messages(
            [
                (
                    "system",
                    """Based on the following information:
                Name: {manga_name}
                Description: {description}
                Reviews: {reviews}
                User Preferences: {manga_information}
                Provide a 0.0/10.0 rating based on the user's preferences and explain why.""",
                ),
                ("user", "Given manga_name: {manga_name}"),
            ]
        )

        return map_chain | synthesis_prompt | self.llm | self.output_parser
