# Examples

This directory contains practical examples of using fauxgen. The examples demonstrate how to use fauxgen's factory methods to simplify testing pandera DataFrameModels.

## Project Structure

```
examples/
├── src/
│   ├── schema/           # pandera DataFrameModel definitions
│   │   ├── user.py      # User model with validation rules
│   │   └── articles.py  # Article model with relationships
│   ├── testing/
│   │   └── fauxgen/   # Generated factory code
│   └── service.py       # Example service using the models
└── tests/
    └── test_service.py  # Test implementations
```

## CLI Usage

Generate factory methods from your pandera DataFrameModel definitions:

```bash
# Generate factory code from models in src directory
uv run fauxgen gen --module-dir src
```

## Example Usage

This example demonstrates how fauxgen simplifies testing by generating factory methods for your pandera DataFrameModels. The factory methods handle field validation rules and allow you to focus on the specific fields relevant to your test.

```python
import pandas as pd
import pytest
from pandera.typing import DataFrame
from src.schema.user import UserSchema
from src.testing.fauxgen.schema.user import user_schema_record

@pytest.mark.parametrize(
    "df_user,df_expected",
    [
        pytest.param(
            pd.DataFrame(
                [
                    user_schema_record(age=18, seed_=1),  # Set specific age for test
                    user_schema_record(age=21, seed_=2),  # Other fields auto-generated
                ]
            ).pipe(DataFrame[UserSchema]),
            pd.DataFrame(
                [
                    user_schema_record(age=21, seed_=2),
                ]
            ).pipe(DataFrame[UserSchema]),
            id="should exclude under 20 users",
        ),
    ],
)
def test_exclude_under_20_users(df_user: DataFrame[UserSchema], df_expected: DataFrame[UserSchema]) -> None:
    df_actual = exclude_under_20_users(df_user)
    pd.testing.assert_frame_equal(df_actual, df_expected)
```

The code above demonstrates:

- Using factory methods to create test data with specific field values
- Automatic generation of valid values for unspecified fields
- Integration with pandera's DataFrame validation
- Reproducible test data using seed values
