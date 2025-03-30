import pandera as pa
from pandera.typing import Series


class UserSchema(pa.DataFrameModel):
    id: Series[int] = pa.Field(ge=1, description="Unique identifier for the user")
    age: Series[int] = pa.Field(ge=0, le=150, description="User's age in years")
    name: Series[str] = pa.Field(nullable=False, description="User's full name")
    email: Series[str] = pa.Field(nullable=True, description="User's contact email address")
    active: Series[bool] = pa.Field(default=True, description="Flag indicating whether the user account is enabled")
