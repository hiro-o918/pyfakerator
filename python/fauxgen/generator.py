import datetime
import math
import random


def gen_int(
    ge: float = 1,
    le: float = 100,
    seed: int | None = None,
) -> int:
    """
    Generate a random integer between ge and le.

    Args:
        ge (float): The lower bound of the range. Defaults to 1.
        le (float): The upper bound of the range. Defaults to 100.
        seed (int, optional): The seed for random number generation. Defaults to None.

    Returns:
        int: A random integer between ge and le.
    """
    if seed is not None:
        random.seed(seed)
    if ge > le:
        raise ValueError("Lower bound 'ge' cannot be greater than upper bound 'le'.")
    return random.randint(int(math.ceil(ge)), int(math.floor(le)))


def gen_float(
    ge: float = 0.0,
    le: float = 100.0,
    seed: int | None = None,
) -> float:
    """
    Generate a random float between ge and le.

    Args:
        ge (float): The lower bound of the range. Defaults to 0.0.
        le (float): The upper bound of the range. Defaults to 100.0.
        seed (int, optional): The seed for random number generation. Defaults to None.

    Returns:
        float: A random float between ge and le.
    """
    if seed is not None:
        random.seed(seed)
    if ge > le:
        raise ValueError("Lower bound 'ge' cannot be greater than upper bound 'le'.")
    return random.uniform(ge, le)


def gen_string(
    min_length: int = 5,
    max_length: int = 10,
    seed: int | None = None,
) -> str:
    """
    Generate a random string of a given length.

    Args:
        min_length (int): The minimum length of the string. Defaults to 5.
        max_length (int): The maximum length of the string. Defaults to 10.
        seed (int, optional): The seed for random number generation. Defaults to None.

    Returns:
        str: A random string of the specified length.
    """
    if seed is not None:
        random.seed(seed)
    length = random.randint(min_length, max_length)
    return "".join(random.choices("abcdefghijklmnopqrstuvwxyz", k=length))


def gen_bool(seed: int | None = None) -> bool:
    """
    Generate a random boolean value.

    Args:
        seed (int, optional): The seed for random number generation. Defaults to None.

    Returns:
        bool: A random boolean value.
    """
    if seed is not None:
        random.seed(seed)
    return random.choice([True, False])


def gen_datetime(
    from_datetime: datetime.datetime | None = None,
    to_datetime: datetime.datetime | None = None,
    seed: int | None = None,
) -> datetime.datetime:
    """
    Generate a random datetime between from_datetime and to_datetime.

    Args:
        from_datetime (datetime.datetime, optional): The start of the range. Defaults to None.
        to_datetime (datetime.datetime, optional): The end of the range. Defaults to None.
        seed (int, optional): The seed for random number generation. Defaults to None.

    Returns:
        datetime.datetime: A random datetime within the specified range.
    """
    if seed is not None:
        random.seed(seed)
    if from_datetime is None:
        from_datetime = datetime.datetime(2020, 1, 1)
    if to_datetime is None:
        to_datetime = datetime.datetime(2021, 1, 1)
    delta = to_datetime - from_datetime
    random_seconds = random.randint(0, int(delta.total_seconds()))
    return from_datetime + datetime.timedelta(seconds=random_seconds)


def gen_date(
    from_date: datetime.date | None = None,
    to_date: datetime.date | None = None,
    seed: int | None = None,
) -> datetime.date:
    """
    Generate a random date between from_date and to_date.

    Args:
        from_date (datetime.date, optional): The start of the range. Defaults to None.
        to_date (datetime.date, optional): The end of the range. Defaults to None.
        seed (int, optional): The seed for random number generation. Defaults to None.

    Returns:
        datetime.date: A random date within the specified range.
    """
    if seed is not None:
        random.seed(seed)
    if from_date is None:
        from_date = datetime.date(2020, 1, 1)
    if to_date is None:
        to_date = datetime.date(2021, 1, 1)
    delta = to_date - from_date
    random_days = random.randint(0, delta.days)
    return from_date + datetime.timedelta(days=random_days)
