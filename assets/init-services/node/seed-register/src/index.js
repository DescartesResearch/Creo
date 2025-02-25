import {registerCollection} from "./db.js";
import assert from "node:assert";
import {randomUser} from "./user.js";

const SEED_COUNT = +(process.env['MG_SEED_COUNT'] ?? 1);
const BATCH_SIZE = 5000;

async function main() {
    await registerCollection.createIndex('username')
    await registerCollection.createIndex('email')

    const ids = Array.from({ length: SEED_COUNT }, (_, i) => i + 1);

    for (let i = 0; i < ids.length; i += BATCH_SIZE) {
        const chunk = ids.slice(i, i + BATCH_SIZE);

        const users = chunk.map(id => randomUser(id))

        const result = await registerCollection.insertMany(users);

        assert(result.acknowledged, 'Failed to insert.')
    }
}

try {
    await main();
    console.log('Seeded register db.')
} catch (e) {
    console.error('Failed to seed register db.')
    console.log(e);
}