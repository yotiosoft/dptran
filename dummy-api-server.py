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

@app.post("/free/v2/translate")
async def translate_for_free(auth_key: str = Form(...), target_lang: str = Form(...), text: List[str] = Form(...), source_lang: Optional[str] = Form(None)):
    print(f"Received request: auth_key={auth_key}, target_lang={target_lang}, text={text}, source_lang={source_lang}")
    source_lang = source_lang.lower() if source_lang else None
    target_lang = target_lang.lower()
    # if source_lang == target_lang, return the text as is
    if source_lang == target_lang:
        return JSONResponse(content={"translations": [TranslationResponseText(text=text) for text in text]})
    # Simulate translation by looking up dummy data
    for text in text:
        for item in dummy_data:
            if ((item.source_lang == source_lang or source_lang == None) and
                    item.target_lang == target_lang and
                    item.request == text):
                return JSONResponse(content={"translations": [TranslationResponseText(text=item.reponse)]})
    return JSONResponse(content={"translations": [TranslationResponseText(text=text) for text in text]})

@app.post("/pro/v2/translate")
async def translate_for_pro(auth_key: str = Form(...), target_lang: str = Form(...), text: List[str] = Form(...), source_lang: Optional[str] = Form(None)):
    print(f"Received request: auth_key={auth_key}, target_lang={target_lang}, source_lang={source_lang}, text={text}")
    source_lang = source_lang.lower() if source_lang else None
    target_lang = target_lang.lower()
    # if source_lang == target_lang, return the text as is
    if source_lang == target_lang:
        return JSONResponse(content={"translations": [TranslationResponseText(text=text) for text in text]})
    # Simulate translation by looking up dummy data
    for text in text:
        for item in dummy_data:
            if ((item.source_lang == source_lang or source_lang == None) and
                    item.target_lang == target_lang and
                    item.request == text):
                return JSONResponse(content={"translations": [TranslationResponseText(text=item.reponse)]})
    return JSONResponse(content={"translations": [TranslationResponseText(text=text) for text in text]})

@app.post("/free/v2/usage")
async def usage_for_free(auth_key: str = Form(...)):
    print(f"Received request: auth_key={auth_key}") 
    return JSONResponse(
        content={
            "character_count": 1000,
            "character_limit": 1000000000000
        }
    )

@app.post("/pro/v2/usage")
async def usage_for_pro(auth_key: str = Form(...)):
    print(f"Received request: auth_key={auth_key}")
    return JSONResponse(
        content={
            "character_count": 10000,
            "character_limit": 1000000000000
        }
    )

class LanguagesResponseElement(BaseModel):
    language: str
    name: str

@app.post("/free/v2/languages")
async def languages_for_free(type: str = Form(...), auth_key: str = Form(...)):
    print(f"Received request: type={type}, auth_key={auth_key}")
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

@app.post("/pro/v2/languages")
async def languages_for_pro(type: str = Form(...), auth_key: str = Form(...)):
    print(f"Received request: type={type}, auth_key={auth_key}")
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
    