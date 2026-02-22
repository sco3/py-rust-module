"""
Pydantic User model equivalent to the Rust implementation
"""
import orjson
from pydantic import BaseModel


class User(BaseModel):
    """A User model with JSON serialization support (Pydantic style)"""
    id: int
    name: str
    email: str
    age: int
    active: bool

    def json(self) -> str:
        """Serialize to JSON string (compact)"""
        return orjson.dumps(self.model_dump()).decode()

    def json_pretty(self) -> str:
        """Serialize to pretty JSON string"""
        return orjson.dumps(self.model_dump(), option=orjson.OPT_INDENT_2).decode()

    @classmethod
    def from_json(cls, json_str: str) -> "User":
        """Create User from JSON string"""
        data = orjson.loads(json_str)
        return cls(**data)

    def dict(self) -> dict:
        """Convert to dictionary"""
        return self.model_dump()

    def model_copy(self, **kwargs) -> "User":
        """Create a copy with modified fields (Pydantic v2 style)"""
        return super().model_copy(update=kwargs)

    def __repr__(self) -> str:
        return f"User(id={self.id}, name='{self.name}', email='{self.email}')"
