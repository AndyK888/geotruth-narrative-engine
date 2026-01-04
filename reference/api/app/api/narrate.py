"""
GeoTruth API - AI Narration Endpoint

Generates fact-checked narration using Gemini (online) or Llama (offline).
"""

import logging
import json

from fastapi import APIRouter, HTTPException
import google.generativeai as genai

from ..config import settings
from ..models import (
    NarrateRequest,
    NarrateResponse,
    Chapter,
)

router = APIRouter()
logger = logging.getLogger(__name__)

# Initialize Gemini if API key available
_gemini_model = None


def get_gemini_model():
    """Get or initialize Gemini model."""
    global _gemini_model
    
    if _gemini_model is None and settings.GEMINI_API_KEY:
        genai.configure(api_key=settings.GEMINI_API_KEY)
        _gemini_model = genai.GenerativeModel('gemini-2.0-flash')
        logger.info("Gemini model initialized")
    
    return _gemini_model


def build_narration_prompt(request: NarrateRequest) -> str:
    """Build prompt for narration generation."""
    
    # Extract key information from truth bundle
    events = request.truth_bundle.events
    
    # Build context from events
    event_descriptions = []
    for i, event in enumerate(events[:20]):  # Limit to first 20 events
        pois = ", ".join([p.name for p in event.pois[:3]]) if event.pois else "No landmarks"
        event_descriptions.append(
            f"- At {event.timestamp.strftime('%H:%M:%S')}: {pois} "
            f"(location: {event.location.lat:.4f}, {event.location.lon:.4f})"
        )
    
    events_text = "\n".join(event_descriptions) or "No events recorded"
    
    # Include transcript if available
    transcript_section = ""
    if request.transcript:
        transcript_section = f"""
## Existing Audio Transcript
{request.transcript[:2000]}
"""
    
    prompt = f"""You are a travel documentary narrator creating engaging, fact-checked content.

## Video Context
This is travel footage with verified GPS and location data. Generate narration that:
1. Only mentions facts that can be verified from the provided data
2. Is engaging and suitable for a travel vlog
3. Follows a natural storytelling flow

## Verified Events and Locations
{events_text}
{transcript_section}
## Output Requirements
Generate a JSON response with this EXACT structure:
{{
  "chapters": [
    {{
      "time_code": "MM:SS",
      "title": "Chapter Title",
      "description": "Brief description"
    }}
  ],
  "script": [
    {{
      "time_code": "MM:SS",
      "narration": "Narration text to speak"
    }}
  ]
}}

Important:
- Each chapter should be 2-5 minutes apart
- Narration should be conversational and engaging
- Only include verifiable facts from the provided data
- Generate 3-5 chapters minimum

Return ONLY valid JSON, no markdown formatting."""

    return prompt


@router.post("/narrate", response_model=NarrateResponse)
async def generate_narration(request: NarrateRequest) -> NarrateResponse:
    """
    Generate AI narration based on verified Truth Bundle.
    
    Uses Gemini 2.0 for online mode. Falls back to placeholder for offline.
    """
    logger.info(
        "Narration requested",
        extra={"context": {
            "event_count": len(request.truth_bundle.events),
            "has_transcript": request.transcript is not None
        }}
    )
    
    model = get_gemini_model()
    
    if model is None:
        # Return placeholder response when no API key
        logger.warning("Gemini API key not configured, returning placeholder")
        return NarrateResponse(
            chapters=[
                Chapter(
                    time_code="00:00",
                    title="Journey Begins",
                    description="Starting our adventure"
                ),
                Chapter(
                    time_code="05:00",
                    title="Main Destination",
                    description="Arriving at the main location"
                ),
            ],
            script=None,
            meta={"engine": "placeholder", "reason": "GEMINI_API_KEY not configured"}
        )
    
    try:
        # Generate with Gemini
        prompt = build_narration_prompt(request)
        
        response = model.generate_content(
            prompt,
            generation_config=genai.GenerationConfig(
                temperature=0.7,
                max_output_tokens=2000,
            )
        )
        
        # Parse JSON response
        text = response.text.strip()
        
        # Clean up if wrapped in markdown
        if text.startswith("```json"):
            text = text[7:]
        if text.startswith("```"):
            text = text[3:]
        if text.endswith("```"):
            text = text[:-3]
        
        data = json.loads(text)
        
        # Build response
        chapters = [
            Chapter(
                time_code=ch.get("time_code", "00:00"),
                title=ch.get("title", "Chapter"),
                description=ch.get("description")
            )
            for ch in data.get("chapters", [])
        ]
        
        script = data.get("script", [])
        
        logger.info(
            "Narration generated",
            extra={"context": {
                "chapters": len(chapters),
                "script_segments": len(script)
            }}
        )
        
        return NarrateResponse(
            chapters=chapters,
            script={"segments": script},
            meta={"engine": "gemini-2.0-flash"}
        )
        
    except json.JSONDecodeError as e:
        logger.error(f"Failed to parse Gemini response: {e}")
        raise HTTPException(status_code=500, detail="Failed to parse AI response")
        
    except Exception as e:
        logger.error(f"Gemini API error: {e}")
        raise HTTPException(status_code=500, detail=f"AI generation failed: {str(e)}")


@router.get("/narrate/status")
async def narration_status():
    """Check AI narration engine status."""
    model = get_gemini_model()
    
    return {
        "online_available": model is not None,
        "offline_available": False,  # Llama integration in Stage 3
        "engine": "gemini-2.0-flash" if model else None
    }
