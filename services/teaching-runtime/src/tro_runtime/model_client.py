"""Private runtime grant; provider credentials remain on the backend."""

from urllib.parse import urlsplit

from agents import OpenAIResponsesModel
from openai import AsyncOpenAI


def create_model(origin: str, grant: str, model: str) -> OpenAIResponsesModel:
    url = urlsplit(origin)
    if (
        url.scheme != "https"
        or not url.hostname
        or url.username
        or url.password
        or url.query
        or url.fragment
        or url.path not in ("", "/")
        or not grant
        or len(grant) > 256
        or not model
        or len(model) > 128
    ):
        raise ValueError("Invalid private model configuration.")
    client = AsyncOpenAI(
        api_key=grant, base_url=origin.rstrip("/") + "/v1", max_retries=0, timeout=20
    )
    return OpenAIResponsesModel(model=model, openai_client=client)
