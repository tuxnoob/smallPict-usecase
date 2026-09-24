from sqlalchemy import Column, String, BigInteger, DateTime, func
from app.database import Base

class Image(Base):
    __tablename__ = "images"
    __table_args__ = {"schema": "public"}

    id = Column(String(50), primary_key=True, index=True)
    filename = Column(String(255), nullable=True)
    url = Column(String, nullable=True)
    size = Column(BigInteger, nullable=True)
    size_origin = Column(BigInteger, nullable=True)
    mime_type = Column(String(100), nullable=True)
    created_at = Column(DateTime(timezone=False), server_default=func.now())

    def to_dict(self):
        return {
            "id": self.id,
            "filename": self.filename,
            "url": self.url,
            "size": self.size,
            "size_origin": self.size_origin,
            "mime_type": self.mime_type,
            "created_at": self.created_at.isoformat() if self.created_at else None
        }
