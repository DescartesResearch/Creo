from typing import Generator


def gen_primes() -> Generator[int, None, None]:
    """Generate an infinite sequence of prime numbers."""
    # Maps composites to primes witnessing their compositeness.
    # This is memory efficient, as the sieve is not "run forward"
    # indefinitely, but only as long as required by the current
    # number being tested.
    D: dict[int, list[int]] = {}

    # The running integer that's checked for primeness
    q: int = 2

    while True:
        if q not in D:
            # q is a new prime.
            # Yield it and mark its first multiple that isn't
            # already marked in previous iterations
            #
            yield q
            D[q * q] = [q]
        else:
            # q is composite. D[q] is the list of primes that
            # divide it. Since we've reached q, we no longer
            # need it in the map, but we'll mark the next
            # multiples of its witnesses to prepare for larger
            # numbers
            #
            for p in D[q]:
                D.setdefault(p + q, []).append(p)
            del D[q]

        q += 1


def generate_first_primes(n: int) -> list[int]:
    if n < 1:
        msg = f"`n` must be greater or equal to `1`, but was {n}."
        raise ValueError(msg)
    gen = gen_primes()
    return [next(gen) for _ in range(n)]
