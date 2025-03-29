import pandera as pa
from pandera.typing import Series


class UserSchema(pa.DataFrameModel):
    id: Series[int] = pa.Field(ge=1)
    age: Series[int] = pa.Field(ge=0, le=150)
    name: Series[str] = pa.Field(nullable=False)
    email: Series[str] = pa.Field(nullable=True)
    active: Series[bool] = pa.Field()
