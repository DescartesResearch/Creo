from typing import Any, Iterable
import asyncio
import os
import itertools
from seed_login import models
from seed_login import db

_SEED_COUNT = int(os.getenv("MG_SEED_COUNT", ""))
_BATCH_SIZE: int = 5000


def batched(iterable: Iterable, n: int):
    """Batch data into tuples of length n. The last batch may be shorter."""
    if n < 1:
        raise ValueError("n must be at least one")
    it = iter(iterable)
    while True:
        chunk_it = itertools.islice(it, n)
        try:
            first_el = next(chunk_it)
        except StopIteration:
            return
        yield itertools.chain((first_el,), chunk_it)


def random_user(id: int) -> dict[str, Any]:
    user = models.User()
    user_dict = user.model_dump()
    user_dict["_id"] = id
    return user_dict


async def seed_database():
    await db.collection.create_index({"username": 1})
    await db.collection.create_index({"email": 1})
    for batch in batched(range(1, _SEED_COUNT + 1), _BATCH_SIZE):
        invoices = [random_user(id) for id in batch]
        insert_result = await db.collection.insert_many(invoices)
        assert insert_result.acknowledged, "Failed to insert"


if __name__ == "__main__":
    asyncio.run(seed_database())
