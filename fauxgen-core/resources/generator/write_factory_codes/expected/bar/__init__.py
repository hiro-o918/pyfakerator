import datetime
from typing import TypedDict

import fauxgen as f


class BarSchemaRecord(TypedDict):
    bar: int


def bar_schema_record(
    *,
    bar: int | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> BarSchemaRecord:
    return {
        "bar": f.Unset.unwrap_or_else(bar, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
    }
