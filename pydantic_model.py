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
        """
        Serialize the model to a compact JSON string.
        
        Returns:
            JSON string of the model with no extra indentation or unnecessary whitespace.
        """
        return orjson.dumps(self.model_dump()).decode()

    def json_pretty(self) -> str:
        """
        Serialize the model to a human-readable JSON string with two-space indentation.
        
        Returns:
            pretty_json (str): Pretty-printed JSON representation of the model using two-space indentation.
        """
        return orjson.dumps(self.model_dump(), option=orjson.OPT_INDENT_2).decode()

    @classmethod
    def from_json(cls, json_str: str) -> "User":
        """
        Create a User instance from a JSON string.
        
        Parameters:
            json_str (str): JSON string representing a User object with keys matching the model fields.
        
        Returns:
            user (User): The constructed User instance.
        """
        data = orjson.loads(json_str)
        return cls(**data)

    def dict(self) -> dict:
        """
        Return a dictionary of the model's field names and values.
        
        Returns:
            dict: Mapping of field names to their corresponding values.
        """
        return self.model_dump()

    def model_copy(self, **kwargs) -> "User":
        """
        Create a new User instance based on this one with specified fields updated.
        
        Parameters:
            **kwargs: Field values to override in the copied model (field name -> new value).
        
        Returns:
            User: A new User instance with the provided updates applied.
        """
        return super().model_copy(update=kwargs)

    def __repr__(self) -> str:
        """
        Return a concise string representation of the User suitable for debugging.
        
        Returns:
            A string in the form "User(id=<id>, name='<name>', email='<email>')" with the instance's id, name, and email substituted.
        """
        return f"User(id={self.id}, name='{self.name}', email='{self.email}')"