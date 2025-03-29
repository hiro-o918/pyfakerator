import pandera as pa
from pandera.typing import Series


class Article(pa.DataFrameModel):
    id: Series[int] = pa.Field(ge=1)
    title: Series[str] = pa.Field(nullable=False)
    content: Series[str] = pa.Field(nullable=True)
    author_id: Series[int] = pa.Field(ge=1)
    published_at: Series[pa.DateTime] = pa.Field(nullable=True)
    is_published: Series[bool] = pa.Field()
