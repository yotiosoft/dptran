# This is a dummy API server for translation services for testing purposes.
# To run this server:
# $ pip3 install -r requirements.txt
# $ uvicorn dummy-api-server:app --reload 

from fastapi import FastAPI, Form
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
    for text in text:
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
async def translate_for_free(auth_key: str = Form(...), target_lang: str = Form(...), text: List[str] = Form(...), source_lang: Optional[str] = Form(None)):
    print(f"Received request: auth_key={auth_key}, target_lang={target_lang}, text={text}, source_lang={source_lang}")
    if auth_key == "":
        return JSONResponse(content={"error": "auth_key is required"}, status_code=400)
    return translate_texts(source_lang, target_lang, text)

@app.post("/pro/v2/translate")
async def translate_for_pro(auth_key: str = Form(...), target_lang: str = Form(...), text: List[str] = Form(...), source_lang: Optional[str] = Form(None)):
    print(f"Received request: auth_key={auth_key}, target_lang={target_lang}, source_lang={source_lang}, text={text}")
    if auth_key == "":
        return JSONResponse(content={"error": "auth_key is required"}, status_code=400)
    return translate_texts(source_lang, target_lang, text)

@app.post("/free/v2/usage")
async def usage_for_free(auth_key: str = Form(...)):
    print(f"Received request: auth_key={auth_key}")
    if auth_key == "":
        return JSONResponse(content={"error": "auth_key is required"}, status_code=400)
    return usage_response(
        character_count=1000,
        character_limit=1000000,
        type="free"
    )

@app.post("/pro/v2/usage")
async def usage_for_pro(auth_key: str = Form(...)):
    print(f"Received request: auth_key={auth_key}")
    if auth_key == "":
        return JSONResponse(content={"error": "auth_key is required"}, status_code=400)
    return usage_response(
        character_count=10000,
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
