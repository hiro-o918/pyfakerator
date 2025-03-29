import datetime
from typing import TypedDict

import fakerator as f


class UserSchemaRecord(TypedDict):
    id: int
    age: int
    name: str
    email: str | None
    active: bool


def user_schema_record(
    *,
    id: int | f.Unset = f.UNSET,
    age: int | f.Unset = f.UNSET,
    name: str | f.Unset = f.UNSET,
    email: str | None | f.Unset = f.UNSET,
    active: bool | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> UserSchemaRecord:
    return {
        "id": f.Unset.unwrap_or_else(id, lambda: f.gen_int(ge=1, le=101, seed=seed_)),
        "age": f.Unset.unwrap_or_else(age, lambda: f.gen_int(ge=0, le=150, seed=seed_)),
        "name": f.Unset.unwrap_or_else(name, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "email": f.Unset.unwrap_or_else(email, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "active": f.Unset.unwrap_or_else(active, lambda: f.gen_bool(seed=seed_)),
    }
