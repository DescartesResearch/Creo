import { invoiceCollection } from "./db.js";
import { randomInvoice } from "./invoice.js";
import assert from "node:assert";

const SEED_COUNT = +(process.env['MG_SEED_COUNT'] ?? 1);
const BATCH_SIZE = 5000;


async function main() {
    const ids = Array.from({ length: SEED_COUNT }, (_, i) => i + 1);

    for (let i = 0; i < ids.length; i += BATCH_SIZE) {
        const chunk = ids.slice(i, i + BATCH_SIZE);

        const invoices = chunk.map(id => randomInvoice(id))

        const result = await invoiceCollection.insertMany(invoices);

        assert(result.acknowledged, 'Failed to insert.')
    }
}

try {
    await main();
    console.log('Seeded invoice db.')
    process.exit(0);
} catch (e) {
    console.error('Failed to seed invoice db.')
    console.log(e);
}