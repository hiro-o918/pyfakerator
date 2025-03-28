import pandera as pa
from pandera.typing import Series


class BarSchema(pa.DataFrameModel):
    bar: Series[int] = pa.Field()
