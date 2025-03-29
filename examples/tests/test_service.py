import pandas as pd
import pytest
from pandera.typing import DataFrame

from src.schema.user import UserSchema
from src.service import exclude_under_20_users
from src.testing.fauxgen.schema.user import user_schema_record


@pytest.mark.parametrize(
    "df_user,df_expected",
    [
        pytest.param(
            pd.DataFrame(
                [
                    user_schema_record(age=18, seed_=1),
                    user_schema_record(age=21, seed_=2),
                ]
            ).pipe(DataFrame[UserSchema]),
            pd.DataFrame(
                [
                    user_schema_record(age=21, seed_=2),
                ]
            ).pipe(DataFrame[UserSchema]),
            id="should exclude under 20 users",
        ),
        pytest.param(
            pd.DataFrame(
                [
                    user_schema_record(age=20, seed_=1),
                    user_schema_record(age=21, seed_=2),
                ]
            ).pipe(DataFrame[UserSchema]),
            pd.DataFrame(
                [
                    user_schema_record(age=20, seed_=1),
                    user_schema_record(age=21, seed_=2),
                ]
            ).pipe(DataFrame[UserSchema]),
            id="should include 20 users",
        ),
    ],
)
def test_exclude_under_20_users(
    df_user: DataFrame[UserSchema], df_expected: DataFrame[UserSchema]
) -> None:
    df_actual = exclude_under_20_users(df_user)
    pd.testing.assert_frame_equal(df_actual, df_expected)
