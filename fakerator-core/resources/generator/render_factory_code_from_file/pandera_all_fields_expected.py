import datetime
from typing import TypedDict

import fakerator as f


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
    int_col: int | f.Unset = f.Unset,
    pa_int_col: int | f.Unset = f.Unset,
    pa_int8_col: int | f.Unset = f.Unset,
    pa_int16_col: int | f.Unset = f.Unset,
    pa_int32_col: int | f.Unset = f.Unset,
    pa_int64_col: int | f.Unset = f.Unset,
    float_col: float | f.Unset = f.Unset,
    pa_float_col: float | f.Unset = f.Unset,
    pa_float16_col: float | f.Unset = f.Unset,
    pa_float32_col: float | f.Unset = f.Unset,
    pa_float64_col: float | f.Unset = f.Unset,
    pa_float128_col: float | f.Unset = f.Unset,
    str_col: str | f.Unset = f.Unset,
    pa_str_col: str | f.Unset = f.Unset,
    bool_col: bool | f.Unset = f.Unset,
    pa_bool_col: bool | f.Unset = f.Unset,
    pa_datetime_col: datetime.datetime | f.Unset = f.Unset,
    pa_date_col: datetime.date | f.Unset = f.Unset,
) -> TestSchemaRecord:
    return {
        "int_col": f.Unset.unwrap_or_else(lambda: f.f.gen_int(ge=0, le=100)),
        "pa_int_col": f.Unset.unwrap_or_else(lambda: f.f.gen_int(ge=0, le=100)),
        "pa_int8_col": f.Unset.unwrap_or_else(lambda: f.f.gen_int(ge=0, le=100)),
        "pa_int16_col": f.Unset.unwrap_or_else(lambda: f.f.gen_int(ge=0, le=100)),
        "pa_int32_col": f.Unset.unwrap_or_else(lambda: f.f.gen_int(ge=0, le=100)),
        "pa_int64_col": f.Unset.unwrap_or_else(lambda: f.f.gen_int(ge=0, le=100)),
        "float_col": f.Unset.unwrap_or_else(lambda: f.f.gen_float(ge=0, le=100)),
        "pa_float_col": f.Unset.unwrap_or_else(lambda: f.f.gen_float(ge=0, le=100)),
        "pa_float16_col": f.Unset.unwrap_or_else(lambda: f.f.gen_float(ge=0, le=100)),
        "pa_float32_col": f.Unset.unwrap_or_else(lambda: f.f.gen_float(ge=0, le=100)),
        "pa_float64_col": f.Unset.unwrap_or_else(lambda: f.f.gen_float(ge=0, le=100)),
        "pa_float128_col": f.Unset.unwrap_or_else(lambda: f.f.gen_float(ge=0, le=100)),
        "str_col": f.Unset.unwrap_or_else(lambda: f.f.gen_string(min_length=5, max_length=10)),
        "pa_str_col": f.Unset.unwrap_or_else(lambda: f.f.gen_string(min_length=5, max_length=10)),
        "bool_col": f.Unset.unwrap_or_else(lambda: f.f.gen_bool()),
        "pa_bool_col": f.Unset.unwrap_or_else(lambda: f.f.gen_bool()),
        "pa_datetime_col": f.Unset.unwrap_or_else(lambda: f.f.gen_datetime(from_datetime=datetime.datetime(2020, 1, 1), to_datetime=datetime.datetime(2021, 1, 1))),
        "pa_date_col": f.Unset.unwrap_or_else(lambda: f.f.gen_date(from_date=datetime.date(2020, 1, 1), to_date=datetime.date(2021, 1, 1))),
    }
