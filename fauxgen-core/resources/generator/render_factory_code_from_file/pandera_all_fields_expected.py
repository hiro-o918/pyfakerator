import datetime
from typing import TypedDict

import fauxgen as f


class TestSchemaRecord(TypedDict):
    int_col: int
    pa_int_col: int
    pa_int8_col: int
    pa_int16_col: int
    pa_int32_col: int
    pa_int64_col: int
    float_col: float
    pa_float_col: float
    pa_float16_col: float
    pa_float32_col: float
    pa_float64_col: float
    pa_float128_col: float
    str_col: str
    pa_str_col: str
    bool_col: bool
    pa_bool_col: bool
    pa_datetime_col: datetime.datetime
    pa_date_col: datetime.date


def test_schema_record(
    *,
    int_col: int | f.Unset = f.UNSET,
    pa_int_col: int | f.Unset = f.UNSET,
    pa_int8_col: int | f.Unset = f.UNSET,
    pa_int16_col: int | f.Unset = f.UNSET,
    pa_int32_col: int | f.Unset = f.UNSET,
    pa_int64_col: int | f.Unset = f.UNSET,
    float_col: float | f.Unset = f.UNSET,
    pa_float_col: float | f.Unset = f.UNSET,
    pa_float16_col: float | f.Unset = f.UNSET,
    pa_float32_col: float | f.Unset = f.UNSET,
    pa_float64_col: float | f.Unset = f.UNSET,
    pa_float128_col: float | f.Unset = f.UNSET,
    str_col: str | f.Unset = f.UNSET,
    pa_str_col: str | f.Unset = f.UNSET,
    bool_col: bool | f.Unset = f.UNSET,
    pa_bool_col: bool | f.Unset = f.UNSET,
    pa_datetime_col: datetime.datetime | f.Unset = f.UNSET,
    pa_date_col: datetime.date | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> TestSchemaRecord:
    return {
        "int_col": f.Unset.unwrap_or_else(int_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "pa_int_col": f.Unset.unwrap_or_else(pa_int_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "pa_int8_col": f.Unset.unwrap_or_else(pa_int8_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "pa_int16_col": f.Unset.unwrap_or_else(pa_int16_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "pa_int32_col": f.Unset.unwrap_or_else(pa_int32_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "pa_int64_col": f.Unset.unwrap_or_else(pa_int64_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "float_col": f.Unset.unwrap_or_else(float_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "pa_float_col": f.Unset.unwrap_or_else(pa_float_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "pa_float16_col": f.Unset.unwrap_or_else(pa_float16_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "pa_float32_col": f.Unset.unwrap_or_else(pa_float32_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "pa_float64_col": f.Unset.unwrap_or_else(pa_float64_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "pa_float128_col": f.Unset.unwrap_or_else(pa_float128_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "str_col": f.Unset.unwrap_or_else(str_col, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "pa_str_col": f.Unset.unwrap_or_else(pa_str_col, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "bool_col": f.Unset.unwrap_or_else(bool_col, lambda: f.gen_bool(seed=seed_)),
        "pa_bool_col": f.Unset.unwrap_or_else(pa_bool_col, lambda: f.gen_bool(seed=seed_)),
        "pa_datetime_col": f.Unset.unwrap_or_else(pa_datetime_col, lambda: f.gen_datetime(from_datetime=datetime.datetime(2020, 1, 1), to_datetime=datetime.datetime(2021, 1, 1), seed=seed_)),
        "pa_date_col": f.Unset.unwrap_or_else(pa_date_col, lambda: f.gen_date(from_date=datetime.date(2020, 1, 1), to_date=datetime.date(2021, 1, 1), seed=seed_)),
    }
