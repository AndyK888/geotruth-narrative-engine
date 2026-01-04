"""
GeoTruth API - Export Endpoints

Export chapters and narration scripts for YouTube and other platforms.
"""

import logging
from typing import List, Optional
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel, Field

router = APIRouter()
logger = logging.getLogger(__name__)


# =============================================================================
# Request/Response Models
# =============================================================================

class Chapter(BaseModel):
    """YouTube chapter."""
    time_code: str
    title: str
    description: Optional[str] = None


class ExportChaptersRequest(BaseModel):
    """Request to export chapters."""
    chapters: List[Chapter]
    format: str = Field(default="youtube", pattern="^(youtube|markdown|srt|json)$")
    include_timestamps: bool = True


class ExportChaptersResponse(BaseModel):
    """Exported chapters result."""
    format: str
    content: str
    filename: str


class ScriptSegment(BaseModel):
    """Narration script segment."""
    time_code: str
    narration: str


class ExportScriptRequest(BaseModel):
    """Request to export narration script."""
    segments: List[ScriptSegment]
    format: str = Field(default="teleprompter", pattern="^(teleprompter|srt|vtt|markdown)$")
    reading_speed_wpm: int = Field(default=150, ge=100, le=250)


class ExportScriptResponse(BaseModel):
    """Exported script result."""
    format: str
    content: str
    filename: str
    estimated_duration_seconds: Optional[float] = None


# =============================================================================
# Chapter Export
# =============================================================================

def format_youtube_chapters(chapters: List[Chapter]) -> str:
    """Format chapters for YouTube description."""
    lines = ["Chapters:"]
    for chapter in chapters:
        lines.append(f"{chapter.time_code} {chapter.title}")
        if chapter.description:
            lines.append(f"  {chapter.description}")
    return "\n".join(lines)


def format_markdown_chapters(chapters: List[Chapter]) -> str:
    """Format chapters as Markdown."""
    lines = ["# Video Chapters\n"]
    for i, chapter in enumerate(chapters, 1):
        lines.append(f"## {i}. {chapter.title} ({chapter.time_code})")
        if chapter.description:
            lines.append(f"\n{chapter.description}\n")
        lines.append("")
    return "\n".join(lines)


def format_srt_chapters(chapters: List[Chapter]) -> str:
    """Format chapters as SRT file."""
    lines = []
    for i, chapter in enumerate(chapters, 1):
        # Parse time code (MM:SS or HH:MM:SS)
        parts = chapter.time_code.split(":")
        if len(parts) == 2:
            start_time = f"00:{parts[0]:0>2}:{parts[1]:0>2},000"
        else:
            start_time = f"{parts[0]:0>2}:{parts[1]:0>2}:{parts[2]:0>2},000"
        
        # Estimate 30 second duration per chapter
        lines.append(str(i))
        lines.append(f"{start_time} --> {start_time[:-4]}30,000")
        lines.append(chapter.title)
        if chapter.description:
            lines.append(chapter.description)
        lines.append("")
    
    return "\n".join(lines)


@router.post("/export/chapters", response_model=ExportChaptersResponse)
async def export_chapters(request: ExportChaptersRequest) -> ExportChaptersResponse:
    """
    Export chapters in various formats.
    
    Supported formats:
    - youtube: Ready for YouTube description
    - markdown: For blog posts
    - srt: Subtitle format
    - json: Raw JSON
    """
    logger.info(
        "Exporting chapters",
        extra={"context": {"format": request.format, "count": len(request.chapters)}}
    )
    
    if not request.chapters:
        raise HTTPException(status_code=400, detail="No chapters provided")
    
    if request.format == "youtube":
        content = format_youtube_chapters(request.chapters)
        filename = "chapters.txt"
    elif request.format == "markdown":
        content = format_markdown_chapters(request.chapters)
        filename = "chapters.md"
    elif request.format == "srt":
        content = format_srt_chapters(request.chapters)
        filename = "chapters.srt"
    else:  # json
        content = [c.model_dump() for c in request.chapters]
        import json
        content = json.dumps(content, indent=2)
        filename = "chapters.json"
    
    return ExportChaptersResponse(
        format=request.format,
        content=content,
        filename=filename
    )


# =============================================================================
# Script Export
# =============================================================================

def time_to_seconds(time_code: str) -> float:
    """Convert time code to seconds."""
    parts = time_code.split(":")
    if len(parts) == 2:
        return int(parts[0]) * 60 + int(parts[1])
    return int(parts[0]) * 3600 + int(parts[1]) * 60 + int(parts[2])


def format_teleprompter(segments: List[ScriptSegment]) -> str:
    """Format script for teleprompter display."""
    lines = []
    for segment in segments:
        lines.append(f"[{segment.time_code}]")
        lines.append("")
        lines.append(segment.narration)
        lines.append("")
        lines.append("---")
        lines.append("")
    return "\n".join(lines)


def format_vtt(segments: List[ScriptSegment]) -> str:
    """Format script as WebVTT."""
    lines = ["WEBVTT", ""]
    
    for i, segment in enumerate(segments):
        start = segment.time_code
        # Estimate end time based on word count
        words = len(segment.narration.split())
        duration_seconds = words / 2.5  # ~150 wpm
        
        # Parse and add duration
        start_seconds = time_to_seconds(start)
        end_seconds = start_seconds + duration_seconds
        
        # Format as HH:MM:SS.mmm
        end_time = f"{int(end_seconds // 3600):02d}:{int((end_seconds % 3600) // 60):02d}:{int(end_seconds % 60):02d}.000"
        start_time = f"00:{start}" if start.count(":") == 1 else start
        start_time += ".000"
        
        lines.append(f"{i + 1}")
        lines.append(f"{start_time} --> {end_time}")
        lines.append(segment.narration)
        lines.append("")
    
    return "\n".join(lines)


@router.post("/export/script", response_model=ExportScriptResponse)
async def export_script(request: ExportScriptRequest) -> ExportScriptResponse:
    """
    Export narration script in various formats.
    
    Supported formats:
    - teleprompter: Easy to read while recording
    - srt: Subtitle format
    - vtt: WebVTT format
    - markdown: For documentation
    """
    logger.info(
        "Exporting script",
        extra={"context": {"format": request.format, "segments": len(request.segments)}}
    )
    
    if not request.segments:
        raise HTTPException(status_code=400, detail="No script segments provided")
    
    # Calculate estimated duration
    total_words = sum(len(s.narration.split()) for s in request.segments)
    estimated_duration = (total_words / request.reading_speed_wpm) * 60
    
    if request.format == "teleprompter":
        content = format_teleprompter(request.segments)
        filename = "script_teleprompter.txt"
    elif request.format == "vtt":
        content = format_vtt(request.segments)
        filename = "script.vtt"
    elif request.format == "srt":
        # Convert VTT to SRT (remove header, use comma instead of dot)
        content = format_vtt(request.segments)
        content = content.replace("WEBVTT\n\n", "").replace(".", ",")
        filename = "script.srt"
    else:  # markdown
        lines = ["# Narration Script\n"]
        for segment in request.segments:
            lines.append(f"## [{segment.time_code}]")
            lines.append(f"\n{segment.narration}\n")
        content = "\n".join(lines)
        filename = "script.md"
    
    return ExportScriptResponse(
        format=request.format,
        content=content,
        filename=filename,
        estimated_duration_seconds=estimated_duration
    )


# =============================================================================
# Full Project Export
# =============================================================================

class FullExportRequest(BaseModel):
    """Request to export complete project."""
    project_id: str
    include_chapters: bool = True
    include_script: bool = True
    include_truth_bundle: bool = True
    format: str = Field(default="zip", pattern="^(zip|folder)$")


@router.post("/export/project")
async def export_project(request: FullExportRequest):
    """
    Export complete project with all assets.
    
    Returns a downloadable ZIP file with:
    - chapters.txt (YouTube format)
    - script.md (Narration script)
    - truth_bundle.json (Verified data)
    - metadata.json (Project info)
    """
    logger.info(
        "Exporting project",
        extra={"context": {"project_id": request.project_id}}
    )
    
    # This would be implemented to:
    # 1. Fetch project from database
    # 2. Generate all export files
    # 3. Create ZIP archive
    # 4. Return file stream
    
    return {
        "status": "pending",
        "message": "Full project export coming in Week 16",
        "project_id": request.project_id
    }
