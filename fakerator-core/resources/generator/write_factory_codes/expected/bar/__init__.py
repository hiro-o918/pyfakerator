import datetime
from typing import TypedDict

import python as f


class BarSchemaRecord(TypedDict):
    bar: int


def bar_schema_record(
    *,
    bar: int | f.Unset = f.Unset,
) -> BarSchemaRecord:
    return {
        "bar": f.Unset.unwrap_or_else(lambda: f.f.gen_int(ge=0, le=100)),
    }
