# This is a dummy API server for translation services for testing purposes.
# To run this server:
# $ pip3 install -r requirements.txt
# $ uvicorn dummy-api-server:app --reload 

from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()

@app.get("/")
async def root():
    return {"message": "Access Successful"}

class TranslationRequest(BaseModel):
    auth_key: str
    target_lang: str
    source_lang: str
    text: list[str]

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
async def translate_for_free(request: TranslationRequest):
    request.source_lang = request.source_lang.lower()
    request.target_lang = request.target_lang.lower()
    # if request.source_lang == request.target_lang, return the text as is
    if request.source_lang == request.target_lang:
        return TranslationResponse(
            translations=[TranslationResponseText(text=text) for text in request.text]
        )
    # Simulate translation by looking up dummy data
    for item in dummy_data:
        if (item.source_lang == request.source_lang and
                item.target_lang == request.target_lang and
                item.request == request.text):
            return TranslationResponse(
                translations=[TranslationResponseText(text=item.reponse)]
            )
    return {"error": "Translation not found"}

@app.post("/pro/v2/translate")
async def translate_for_pro(request: TranslationRequest):
    request.source_lang = request.source_lang.lower()
    request.target_lang = request.target_lang.lower()
    # if request.source_lang == request.target_lang, return the text as is
    if request.source_lang == request.target_lang:
        return TranslationResponse(
            translations=[TranslationResponseText(text=text) for text in request.text]
        )
    # Simulate translation by looking up dummy data
    for item in dummy_data:
        if (item.source_lang == request.source_lang and
                item.target_lang == request.target_lang and
                item.request == request.text):
            return TranslationResponse(
                translations=[TranslationResponseText(text=item.reponse)]
            )
    return {"error": "Translation not found"}

class UsageRequest(BaseModel):
    auth_key: str

class UsageResponse(BaseModel):
    charactor_count: int
    charactor_limit: int

@app.post("/free/v2/usage")
async def usage_for_free(request: UsageRequest):
    return UsageResponse(
        charactor_count=1000,
        charactor_limit=500000
    )

@app.post("/pro/v2/usage")
async def usage_for_pro(request: UsageRequest):
    return UsageResponse(
        charactor_count=10000,
        charactor_limit=1000000000000
    )

class LanguagesRequest(BaseModel):
    type: str
    auth_key: str

class LanguagesResponseElement(BaseModel):
    language: str
    name: str

@app.post("/free/v2/languages")
async def languages_for_free(request: LanguagesRequest):
    if request.type == "source":
        return [
            LanguagesResponseElement(language="EN", name="English"),
            LanguagesResponseElement(language="JA", name="Japanese"),
            LanguagesResponseElement(language="FR", name="French"),
        ]
    elif request.type == "target":
        return [
            LanguagesResponseElement(language="EN", name="English"),
            LanguagesResponseElement(language="JA", name="Japanese"),
            LanguagesResponseElement(language="FR", name="French"),
        ]
    
@app.post("/pro/v2/languages")
async def languages_for_pro(request: LanguagesRequest):
    if request.type == "source":
        return [
            LanguagesResponseElement(language="EN", name="English"),
            LanguagesResponseElement(language="JA", name="Japanese"),
            LanguagesResponseElement(language="FR", name="French"),
        ]
    elif request.type == "target":
        return [
            LanguagesResponseElement(language="EN", name="English"),
            LanguagesResponseElement(language="JA", name="Japanese"),
            LanguagesResponseElement(language="FR", name="French"),
        ]
