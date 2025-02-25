import { userCollection } from "./db.js";
import {randomUser} from "./user.js";
import assert from "node:assert";

const SEED_COUNT = +(process.env['MG_SEED_COUNT'] ?? 1);
const BATCH_SIZE = 5000;


async function main() {
    const ids = Array.from({ length: SEED_COUNT }, (_, i) => i + 1);

    for (let i = 0; i < ids.length; i += BATCH_SIZE) {
        const chunk = ids.slice(i, i + BATCH_SIZE);

        const users = chunk.map(id => randomUser(id))

        const result = await userCollection.insertMany(users);

        assert(result.acknowledged, 'Failed to insert.')
    }
}

try {
    await main();
    console.log('Seeded user db.')
} catch (e) {
    console.error('Failed to seed user db.')
    console.log(e);
}