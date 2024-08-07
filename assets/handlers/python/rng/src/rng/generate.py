import random


def yield_random_number(min: int, max: int):
    """Yields a random number inside the specified range of 'min' and 'max'."""
    while True:
        yield random.randint(min, max)


def generate_random_numbers(n: int, min: int, max: int) -> list[int]:
    if n < 1:
        msg = f"`n` must be greater or equal to `1`, but was {n}"
        raise ValueError(msg)
    if min > max:
        # Swap min and max
        min += max
        max = min - max
        min = min - max
    gen = yield_random_number(min=min, max=max)
    return [next(gen) for _ in range(n)]
