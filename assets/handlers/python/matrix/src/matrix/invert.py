import numpy as np
from numpy import linalg


def invert_random_matrix(size: int) -> list[list[int]]:
    """Generates a random square matrix of the given size and inverts it."""

    if size < 1:
        msg = f"Matrix size must be greater or equal to `1`, but was {size}."
        raise ValueError(msg)

    matrix = np.random.random(size=(size, size))
    return linalg.inv(matrix).tolist()
