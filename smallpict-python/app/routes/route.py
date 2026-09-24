from typing import Optional
from app.config import settings
from fastapi import APIRouter, status, Depends, UploadFile, File, Form, Request, HTTPException
from sqlalchemy.orm import Session
from app.database import get_db
from app.helper.smallpict import upload
from datetime import datetime
from fastapi.responses import JSONResponse
from app.database import check_db_connection

router = APIRouter()

@router.get("/")
def get_base_status():
    return {
        "status": "ok",
        "service": settings.APP_NAME,
        "version": "1.0.0"
    }

@router.get("/health")
def health_check():
    db_ok = check_db_connection()
    status_code = status.HTTP_200_OK if db_ok else status.HTTP_503_SERVICE_UNAVAILABLE

    return JSONResponse(
        status_code=status_code,
        content={
            "status": "ok" if db_ok else "unhealthy",
            "database": "connected" if db_ok else "disconnected",
            "timestamp": datetime.utcnow().isoformat()
        }
    )

@router.post("/upload")
async def handle_upload(
    request: Request,
    db: Session = Depends(get_db),
    file: Optional[UploadFile] = File(None),
    image: Optional[UploadFile] = File(None),
    name: Optional[str] = Form(None),
    filename: Optional[str] = Form(None)
):
    content_type = request.headers.get("content-type", "").lower()

    if "application/json" in content_type:
        try:
            body = await request.json()
        except Exception:
            raise HTTPException(status_code=400, detail="Invalid JSON payload")

        base64_str = body.get("image") or body.get("base64") or body.get("data")
        file_name = body.get("name") or body.get("filename")

        if not base64_str:
            raise HTTPException(status_code=400, detail="Field 'image' or 'base64' is required in JSON payload")

        try:
            img_record = await upload(
                db=db,
                base64_str=base64_str,
                original_filename=file_name
            )
            return {
                "status": "success",
                "message": "Image uploaded successfully",
                "data": img_record.to_dict()
            }
        except Exception as e:
            raise HTTPException(status_code=500, detail=str(e))

    upload_target = file or image
    if upload_target:
        file_bytes = await upload_target.read()
        target_filename = name or filename or upload_target.filename
        try:
            img_record = await upload(
                db=db,
                file_bytes=file_bytes,
                original_filename=target_filename
            )
            return {
                "status": "success",
                "message": "Image uploaded successfully",
                "data": img_record.to_dict()
            }
        except Exception as e:
            raise HTTPException(status_code=500, detail=str(e))

    form = await request.form()
    base64_str = form.get("image") or form.get("base64")
    if base64_str and isinstance(base64_str, str):
        target_filename = form.get("name") or form.get("filename")
        try:
            img_record = await upload(
                db=db,
                base64_str=base64_str,
                original_filename=target_filename
            )
            return {
                "status": "success",
                "message": "Image uploaded successfully",
                "data": img_record.to_dict()
            }
        except Exception as e:
            raise HTTPException(status_code=500, detail=str(e))

    raise HTTPException(status_code=400, detail="No file or image payload found in request")
