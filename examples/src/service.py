from pandera.typing import DataFrame

from src.schema.user import UserSchema


def exclude_under_20_users(df_users: DataFrame[UserSchema]) -> DataFrame[UserSchema]:
    """Exclude users under 20 years old from the DataFrame.
    """
    return df_users[df_users["age"] >= 20].reset_index(drop=True).pipe(DataFrame[UserSchema])
