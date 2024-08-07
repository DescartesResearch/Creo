from typing import Any, Iterable
import asyncio
import os
import itertools
from seed_invoice import models
from seed_invoice import db

_SEED_COUNT = int(os.getenv("MG_SEED_COUNT", ""))
_BATCH_SIZE: int = 50000


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


def random_invoice(id: int) -> dict[str, Any]:
    invoice = models.Invoice()
    invoice_dict = invoice.model_dump()
    invoice_dict["_id"] = id
    return invoice_dict


async def seed_database(r: Iterable):
    collection = db.get_collection()
    for batch in batched(r, _BATCH_SIZE):
        invoices = [random_invoice(id) for id in batch]
        insert_result = await collection.insert_many(invoices)
        assert insert_result.acknowledged, "Failed to insert"


def main():
    asyncio.run(seed_database(range(1, _SEED_COUNT + 1)))


if __name__ == "__main__":
    main()
