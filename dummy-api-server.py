# This is a dummy API server for translation services for testing purposes.
# To run this server:
# $ pip3 install -r requirements.txt
# $ uvicorn dummy-api-server:app --reload 

from fastapi import FastAPI, Form, Request
from fastapi.responses import JSONResponse
from pydantic import BaseModel
from typing import List, Optional

app = FastAPI()

@app.get("/")
async def root():
    return {"message": "Access Successful"}

class TranslationResponseText(BaseModel):
    text: str

class TranslationResponse(BaseModel):
    translations: list[TranslationResponseText]

class TranslationDummyData(BaseModel):
    source_lang: str
    target_lang: str
    request: str
    reponse: str

class SplitSentences(str):
    pass

class Formality(str):
    pass

class ModelType(str):
    pass

class TagHandling(str):
    pass

class TranslateRequest(BaseModel):
    text: List[str]
    target_lang: str
    source_lang: Optional[str] = None
    context: Optional[str] = None
    show_billed_characters: Optional[bool] = None
    split_sentences: Optional[SplitSentences] = None
    preserve_formatting: Optional[bool] = None
    formality: Optional[Formality] = None
    model_type: Optional[ModelType] = None
    glossary_id: Optional[str] = None
    tag_handling: Optional[TagHandling] = None
    outline_detection: Optional[bool] = None
    non_splitting_tags: Optional[List[str]] = None
    splitting_tags: Optional[List[str]] = None
    ignore_tags: Optional[List[str]] = None

dummy_data = [
    TranslationDummyData(
        source_lang="en",
        target_lang="ja",
        request="Hello",
        reponse="こんにちは"
    ),
    TranslationDummyData(
        source_lang="ja",
        target_lang="en",
        request="こんにちは",
        reponse="Hello"
    ),
    TranslationDummyData(
        source_lang="en",
        target_lang="ja",
        request="Hello, World!",
        reponse="ハロー、ワールド！"
    ),
    TranslationDummyData(
        source_lang="ja",
        target_lang="en",
        request="ハロー、ワールド！",
        reponse="Hello, World!"
    ),
    TranslationDummyData(
        source_lang="en",
        target_lang="fr",
        request="Hello",
        reponse="Bonjour"
    ),
    TranslationDummyData(
        source_lang="fr",
        target_lang="en",
        request="Bonjour",
        reponse="Hello"
    ),
    TranslationDummyData(
        source_lang="ja",
        target_lang="fr",
        request="こんにちは",
        reponse="Bonjour"
    ),
    TranslationDummyData(
        source_lang="fr",
        target_lang="ja",
        request="Bonjour",
        reponse="こんにちは"
    ),
]

character_count = 0

def translate_texts(source_lang: str, target_lang: str, text: str) -> str:
    source_lang = source_lang.lower() if source_lang else None
    target_lang = target_lang.lower()
    # if source_lang == target_lang, return the text as is
    if source_lang == target_lang:
        return JSONResponse(content={"translations": [
            {
                "text": text
            }
        ]})
    # Simulate translation by looking up dummy data
    results = []
    global character_count
    for text in text:
        character_count += len(text)
        for item in dummy_data:
            if ((item.source_lang == source_lang or source_lang == None) and
                    item.target_lang == target_lang and
                    item.request == text):
                results.append({
                    "text": item.reponse
                })
    if results:
        return JSONResponse(content={"translations": results})
    return JSONResponse(content={"translations": [
        {
            "text": text
        }
    ]})

def usage_response(character_count: int, character_limit: int, type: str) -> JSONResponse:
    if type == "free":
        return JSONResponse(
            content={
                "character_count": character_count,
                "character_limit": character_limit
            }
        )
    elif type == "pro":
        return JSONResponse(
            content={
                "character_count": character_count * 10,
                "character_limit": character_limit * 10
            }
        )
    return JSONResponse(
        content={
            "error": "Invalid type"
        }
    )

def languages_response(type: str) -> JSONResponse:
    if type == "source":
        return JSONResponse(
            content=[
                {
                    "language": "EN",
                    "name": "English"
                },
                {
                    "language": "EN-US",
                    "name": "English"
                },
                {
                    "language": "JA",
                    "name": "Japanese"
                },
                {
                    "language": "FR",
                    "name": "French"
                }
            ]
        )
    elif type == "target":
        return JSONResponse(
            content=[
                {
                    "language": "EN",
                    "name": "English"
                },
                {
                    "language": "EN-US",
                    "name": "English"
                },
                {
                    "language": "JA",
                    "name": "Japanese"
                },
                {
                    "language": "FR",
                    "name": "French"
                }
            ]
        )
    return JSONResponse(
        content={
            "error": "Invalid type"
        }
    )

@app.post("/free/v2/translate")
async def translate_for_free(request: Request, body: TranslateRequest):
    # Get the Authorization header
    auth_header = request.headers.get("Authorization", "")
    print(f"Authorization header: {auth_header}")

    # Check if the Authorization header is valid
    if not auth_header.startswith("DeepL-Auth-Key "):
        return JSONResponse(content={"error": "Invalid or missing Authorization header"}, status_code=401)

    api_key = auth_header.replace("DeepL-Auth-Key ", "")
    if not api_key:
        return JSONResponse(content={"error": "auth_key is required"}, status_code=400)

    # Log the request body for debugging
    print(f"Received JSON: {body.json()}")

    # Call the translation function
    return translate_texts(body.source_lang, body.target_lang, body.text)

@app.post("/pro/v2/translate")
async def translate_for_pro(request: Request, body: TranslateRequest):
    # Get the Authorization header
    auth_header = request.headers.get("Authorization", "")
    print(f"Authorization header: {auth_header}")

    # Check if the Authorization header is valid
    if not auth_header.startswith("DeepL-Auth-Key "):
        return JSONResponse(content={"error": "Invalid or missing Authorization header"}, status_code=401)

    api_key = auth_header.replace("DeepL-Auth-Key ", "")
    if not api_key:
        return JSONResponse(content={"error": "auth_key is required"}, status_code=400)

    # Log the request body for debugging
    print(f"Received JSON: {body.json()}")

    # Call the translation function
    return translate_texts(body.source_lang, body.target_lang, body.text)

@app.post("/free/v2/usage")
async def usage_for_free(auth_key: str = Form(...)):
    print(f"Received request: auth_key={auth_key}")
    if auth_key == "":
        return JSONResponse(content={"error": "auth_key is required"}, status_code=400)
    global character_count
    return usage_response(
        character_count=character_count,
        character_limit=1000000,
        type="free"
    )

@app.post("/pro/v2/usage")
async def usage_for_pro(auth_key: str = Form(...)):
    print(f"Received request: auth_key={auth_key}")
    if auth_key == "":
        return JSONResponse(content={"error": "auth_key is required"}, status_code=400)
    global character_count
    return usage_response(
        character_count=character_count,
        character_limit=1000000000000,
        type="pro"
    )

class LanguagesResponseElement(BaseModel):
    language: str
    name: str

@app.post("/free/v2/languages")
async def languages_for_free(type: str = Form(...), auth_key: str = Form(...)):
    print(f"Received request: type={type}, auth_key={auth_key}")
    if auth_key == "":
        return JSONResponse(content={"error": "auth_key is required"}, status_code=400)
    return languages_response(type)

@app.post("/pro/v2/languages")
async def languages_for_pro(type: str = Form(...), auth_key: str = Form(...)):
    print(f"Received request: type={type}, auth_key={auth_key}")
    if auth_key == "":
        return JSONResponse(content={"error": "auth_key is required"}, status_code=400)
    return languages_response(type)
