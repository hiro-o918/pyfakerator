import pandera as pa
from pandera.typing import Series


class TestSchema(pa.DataFrameModel):
    int_col: Series[int] = pa.Field(ge=0.1, le=10.0)
    float_col_only_ge: Series[float] = pa.Field(ge=10)
    float_col_only_le: Series[float] = pa.Field(le=10)
    pa_bool_col: Series[pa.Bool] = pa.Field(nullable=True)
