import datetime
from typing import TypedDict

import fakerator as f


class FooSchemaRecord(TypedDict):
    foo: int


def foo_schema_record(
    *,
    foo: int | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> FooSchemaRecord:
    return {
        "foo": f.Unset.unwrap_or_else(foo, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
    }
