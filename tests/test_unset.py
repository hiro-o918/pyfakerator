from typing import Any, Callable
from fauxgen.unset import Unset, UNSET
import pytest


class TestUnset:
    @pytest.mark.parametrize(
        "value, func, expected",
        [
            pytest.param(UNSET, lambda: 42, 42),
            pytest.param(UNSET, lambda: "default", "default"),
            pytest.param(42, lambda: 100, 42),
        ],
    )
    def test_unwrap_or_else(self, value: Unset | Any , func: Callable[[], Any], expected: Any) -> None:
        actual = Unset.unwrap_or_else(value, func)
        assert actual    == expected
