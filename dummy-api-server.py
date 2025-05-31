# This is a dummy API server for translation services for testing purposes.
# To run this server:
# $ pip3 install -r requirements.txt
# $ uvicorn dummy-api-server:app --reload 

from fastapi import FastAPI, Query
from pydantic import BaseModel

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
async def translate_for_free(auth_key: str, target_lang: str, text: list[str], source_lang = Query(None)):
    print(f"Received request: auth_key={auth_key}, target_lang={target_lang}, text={text}, source_lang={source_lang}")
    source_lang = source_lang.lower()
    target_lang = target_lang.lower()
    # if source_lang == target_lang, return the text as is
    if source_lang == target_lang:
        return TranslationResponse(
            translations=[TranslationResponseText(text=text) for text in text]
        )
    # Simulate translation by looking up dummy data
    for item in dummy_data:
        if (item.source_lang == source_lang and
                item.target_lang == target_lang and
                item.request == text):
            return TranslationResponse(
                translations=[TranslationResponseText(text=item.reponse)]
            )
    return {"error": "Translation not found"}

@app.post("/pro/v2/translate")
async def translate_for_pro(auth_key: str, target_lang: str, source_lang: str, text: list[str]):
    print(f"Received request: auth_key={auth_key}, target_lang={target_lang}, source_lang={source_lang}, text={text}")
    source_lang = source_lang.lower()
    target_lang = target_lang.lower()
    # if source_lang == target_lang, return the text as is
    if source_lang == target_lang:
        return TranslationResponse(
            translations=[TranslationResponseText(text=text) for text in text]
        )
    # Simulate translation by looking up dummy data
    for item in dummy_data:
        if (item.source_lang == source_lang and
                item.target_lang == target_lang and
                item.request == text):
            return TranslationResponse(
                translations=[TranslationResponseText(text=item.reponse)]
            )
    return {"error": "Translation not found"}

class UsageResponse(BaseModel):
    charactor_count: int
    charactor_limit: int

@app.post("/free/v2/usage")
async def usage_for_free(auth_key: str):
    print(f"Received request: auth_key={auth_key}") 
    return UsageResponse(
        charactor_count=1000,
        charactor_limit=500000
    )

@app.post("/pro/v2/usage")
async def usage_for_pro(auth_key: str):
    print(f"Received request: auth_key={auth_key}") 
    return UsageResponse(
        charactor_count=10000,
        charactor_limit=1000000000000
    )

class LanguagesResponseElement(BaseModel):
    language: str
    name: str

@app.post("/free/v2/languages")
async def languages_for_free(type: str, auth_key: str):
    print(f"Received request: type={type}, auth_key={auth_key}")
    if type == "source":
        return [
            LanguagesResponseElement(language="EN", name="English"),
            LanguagesResponseElement(language="JA", name="Japanese"),
            LanguagesResponseElement(language="FR", name="French"),
        ]
    elif type == "target":
        return [
            LanguagesResponseElement(language="EN", name="English"),
            LanguagesResponseElement(language="JA", name="Japanese"),
            LanguagesResponseElement(language="FR", name="French"),
        ]
    
@app.post("/pro/v2/languages")
async def languages_for_pro(type: str, auth_key: str):
    print(f"Received request: type={type}, auth_key={auth_key}")
    if type == "source":
        return [
            LanguagesResponseElement(language="EN", name="English"),
            LanguagesResponseElement(language="JA", name="Japanese"),
            LanguagesResponseElement(language="FR", name="French"),
        ]
    elif type == "target":
        return [
            LanguagesResponseElement(language="EN", name="English"),
            LanguagesResponseElement(language="JA", name="Japanese"),
            LanguagesResponseElement(language="FR", name="French"),
        ]
