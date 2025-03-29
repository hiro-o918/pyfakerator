from typing import Callable, TypeVar

T = TypeVar("T")


class Unset:
    @classmethod
    def unwrap_or_else(cls, v: T | "Unset", func: Callable[[], T]) -> T:
        if isinstance(v, Unset):
            return func()
        return v


UNSET = Unset()
