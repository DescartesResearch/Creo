import { faker } from "@faker-js/faker";
import {ObjectId} from "mongodb";


export function randomUser(id) {
    return {
        _id: new ObjectId(id),
        username: faker.string.alphanumeric({ length: { min: 3, max: 64 } }),
        email: faker.string.alphanumeric({ length: { min: 3, max: 64 } }),
        created_at: Date.now(),
        password_hash: Buffer.from(faker.string.alphanumeric({ length: 97 })),
    }
}