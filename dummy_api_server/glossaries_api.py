from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List
import uuid
from datetime import datetime

router = APIRouter(prefix="/v3", tags=["glossaries"])

GLOSSARIES = {}

class GlossaryDictionary(BaseModel):
    source_lang: str
    target_lang: str
    entries: str
    entries_format: str

class GlossaryCreateRequest(BaseModel):
    name: str
    dictionaries: List[GlossaryDictionary]

class GlossaryDictionaryResponse(BaseModel):
    source_lang: str
    target_lang: str
    entry_count: int

class GlossaryResponse(BaseModel):
    glossary_id: str
    name: str
    dictionaries: List[GlossaryDictionaryResponse]
    creation_time: str

class GlossaryListResponse(BaseModel):
    glossaries: List[GlossaryResponse]

class GlossaryLangPair(BaseModel):
    source_lang: str
    target_lang: str

class GlossaryLangPairsResponse(BaseModel):
    supported_languages: List[GlossaryLangPair]

@router.post("/free/glossaries", response_model=GlossaryResponse)
async def create_glossary(req: GlossaryCreateRequest):
    glossary_id = str(uuid.uuid4())
    creation_time = datetime.utcnow().isoformat() + "Z"
    dict_responses = []
    for d in req.dictionaries:
        entry_count = len([line for line in d.entries.split("\n") if line.strip()])
        dict_responses.append(
            GlossaryDictionaryResponse(
                source_lang=d.source_lang,
                target_lang=d.target_lang,
                entry_count=entry_count
            )
        )
    glossary = GlossaryResponse(
        glossary_id=glossary_id,
        name=req.name,
        dictionaries=dict_responses,
        creation_time=creation_time
    )
    GLOSSARIES[glossary_id] = glossary
    return glossary

@router.post("/pro/glossaries/{glossary_id}", response_model=GlossaryResponse)
async def create_pro_glossary(req: GlossaryCreateRequest):
    return await create_glossary(req)


@router.get("/free/glossaries", response_model=GlossaryListResponse)
async def list_glossaries():
    return GlossaryListResponse(glossaries=list(GLOSSARIES.values()))

@router.get("/pro/glossaries", response_model=GlossaryListResponse)
async def list_pro_glossaries():
    return GlossaryListResponse(glossaries=list(GLOSSARIES.values()))


@router.delete("/free/glossaries/{glossary_id}")
async def delete_glossary(glossary_id: str):
    if glossary_id not in GLOSSARIES:
        raise HTTPException(status_code=404, detail="Glossary not found")
    del GLOSSARIES[glossary_id]
    return {"status": "deleted"}

@router.delete("/pro/glossaries/{glossary_id}")
async def delete_pro_glossary(glossary_id: str):
    return await delete_glossary(glossary_id)


@router.patch("/free/glossaries/{glossary_id}")
async def patch_glossary(glossary_id: str, req: GlossaryCreateRequest):
    if glossary_id not in GLOSSARIES:
        raise HTTPException(status_code=404, detail="Glossary not found")
    glossary = GLOSSARIES[glossary_id]
    glossary.name = req.name
    # Add to the dictionaries
    glossary.dictionaries.extend(req.dictionaries)
    # If there are duplicate language pairs, just overwrite by the new one
    unique_dicts = {}
    for d in glossary.dictionaries:
        key = (d.source_lang, d.target_lang)
        unique_dicts[key] = d
    glossary.dictionaries = list(unique_dicts.values())
    GLOSSARIES[glossary_id] = glossary
    return {"status": "updated"}

@router.patch("/pro/glossaries/{glossary_id}")
async def patch_pro_glossary(glossary_id: str, req: GlossaryCreateRequest):
    return await patch_glossary(glossary_id, req)

@router.get("/free/glossary-language-pairs", response_model=GlossaryLangPairsResponse)
async def get_glossary_language_pairs():
    return GlossaryLangPairsResponse(
        supported_languages=[
            GlossaryLangPair(source_lang="EN", target_lang="FR"),
            GlossaryLangPair(source_lang="DE", target_lang="EN"),
            GlossaryLangPair(source_lang="EN", target_lang="JA"),
        ]
    )

@router.get("/pro/glossary-language-pairs", response_model=GlossaryLangPairsResponse)
async def get_pro_glossary_language_pairs():
    return await get_glossary_language_pairs()
