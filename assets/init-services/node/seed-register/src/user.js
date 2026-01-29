import { faker } from "@faker-js/faker";

export function randomUser(id) {
    return {
        _id: id,
        username: faker.string.alphanumeric({ length: { min: 3, max: 64 } }),
        email: faker.string.alphanumeric({ length: { min: 3, max: 64 } }),
        password: Buffer.from(faker.string.alphanumeric({ length: 97 })),
        created_at: Date.now(),
    }
}