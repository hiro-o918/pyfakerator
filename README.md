# fauxgen

A factory method generator that streamlines test data creation by automating field value generation.

## Overview

Testing classes with numerous fields often requires extensive boilerplate code. `fauxgen` addresses this challenge by automatically generating factory methods from class definitions, significantly reducing test code complexity.

Currently supports `pandera.DataFrameModel` with plans for future expansions.

## Quick Start

```bash
uv add fauxgen
uv run fauxgen gen --module-dir <your_module>
```

## Usage Example

Example project can be found in the [example directory](./example).

### Traditional Approach (Without fauxgen)

Testing DataFrame models traditionally requires explicit specification of all fields, even when testing a single validation:

```python
import pandera as pa
from pandera.typing import Series

class UserSchema(pa.DataFrameModel):
    id: Series[int] = pa.Field(ge=1)
    age: Series[int] = pa.Field(ge=0, le=150)
    name: Series[str] = pa.Field()
    email: Series[str] = pa.Field(nullable=True)
    active: Series[bool] = pa.Field()

def test_user_registration():
    # Forced to specify every field when testing age validation
    df_user = pd.DataFrame([
        {
            "id": 1,            # Unrelated to test
            "age": 151,         # Actual test target
            "name": "test",     # Unrelated to test
            "email": "test@example.com",  # Unrelated to test
            "active": True,     # Unrelated to test
        },
    ]).pipe(DataFrame[UserSchema])
    # Test age validation...
```

### Simplified Testing (With fauxgen)

fauxgen generates factory methods that enable focused testing by automatically handling irrelevant fields:

```python
from .factories import user_schema_record  # Generated by fauxgen

def test_user_registration():
    # Focus solely on the field under test
    # Other fields are automatically populated with valid values
    df_user = pd.DataFrame([
        user_schema_record(age=151), # Set specific age for test
    ]).pipe(DataFrame[UserSchema])  # Test age validation...

def test_user_email_optional():
    # Effortlessly test specific scenarios
    # without worrying about irrelevant fields
    df_user = pd.DataFrame([
        user_schema_record(email=None)  # Set email to None for test
    ]).pipe(DataFrame[UserSchema])  # Test nullable email...
```

## How It Works

### Input: pandera.DataFrameModel Definition

```python
import pandera as pa
from pandera.typing import Series

class TestSchema(pa.DataFrameModel):
    int_col: Series[int] = pa.Field(ge=0.1, le=10.0)
    float_col_only_ge: Series[float] = pa.Field(ge=10)
    float_col_only_le: Series[float] = pa.Field(le=10)
    pa_bool_col: Series[pa.Bool] = pa.Field(nullable=True)
```

### Output: Generated Factory Method

```python
import fauxgen as f

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
```

### Key Features

1. **Enhanced Type Safety**:

   - Leverages TypedDict for comprehensive type checking
   - Provides full IDE support with type hints and autocompletion

2. **Intelligent Field Generation**:

   - Selectively override specific fields while auto-generating others
   - Maintains data integrity through validation-aware value generation

3. **Validation-Aware Generation**:
   - Automatically respects field constraints (`ge`, `le`, etc.)
   - Properly handles optional fields with `nullable` support
