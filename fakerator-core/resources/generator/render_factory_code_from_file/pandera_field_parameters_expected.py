import datetime
from typing import TypedDict

import fakerator as f


class TestSchemaRecord(TypedDict):
    int_col: int
    float_col_only_ge: float
    float_col_only_le: float
    pa_bool_col: bool | None


def test_schema_record(
    *,
    int_col: int | f.Unset = f.UNSET,
    float_col_only_ge: float | f.Unset = f.UNSET,
    float_col_only_le: float | f.Unset = f.UNSET,
    pa_bool_col: bool | None | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> TestSchemaRecord:
    return {
        "int_col": f.Unset.unwrap_or_else(int_col, lambda: f.gen_int(ge=0.1, le=10, seed=seed_)),
        "float_col_only_ge": f.Unset.unwrap_or_else(float_col_only_ge, lambda: f.gen_float(ge=10, le=110, seed=seed_)),
        "float_col_only_le": f.Unset.unwrap_or_else(float_col_only_le, lambda: f.gen_float(ge=-90, le=10, seed=seed_)),
        "pa_bool_col": f.Unset.unwrap_or_else(pa_bool_col, lambda: f.gen_bool(seed=seed_)),
    }
