import argon2

TIME_COST: int = 1
MEMORY_COST: int = 6144
SALT_LEN: int = 16

hasher = argon2.PasswordHasher(
    time_cost=TIME_COST,
    memory_cost=MEMORY_COST,
    salt_len=SALT_LEN,
)


def hash_password(
    password: str,
) -> str:
    """Computes the hash of the given password using the Argon2 hash function."""

    return hasher.hash(password)
