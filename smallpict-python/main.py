import uvicorn
from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
import os
from app.config import settings
from app.database import init_db
from app.routes import route

app = FastAPI(title=settings.APP_NAME, version="1.0.0")

os.makedirs(settings.UPLOAD_PATH, exist_ok=True)
app.mount("/uploads", StaticFiles(directory=settings.UPLOAD_PATH), name="uploads")

app.include_router(route.router)

@app.on_event("startup")
def on_startup():
    init_db()

if __name__ == "__main__":
    uvicorn.run("main:app", host=settings.APP_HOST, port=settings.APP_PORT, reload=True)
