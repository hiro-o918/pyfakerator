import pandera as pa
from pandera.typing import Series


class FooSchema(pa.DataFrameModel):
    foo: Series[int] = pa.Field()
