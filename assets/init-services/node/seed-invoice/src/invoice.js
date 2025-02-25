import { faker } from "@faker-js/faker";
import {ObjectId} from "mongodb";

function randomAddress() {
    return {
        first_name: faker.string.alphanumeric({ length: { min: 2, max: 64 } }),
        last_name: faker.string.alphanumeric({ length: { min: 2, max: 64 } }),
        street: faker.string.alphanumeric({ length: { min: 2, max: 128 } }),
        city: faker.string.alphanumeric({ length: { min: 3, max: 64 } }),
        country: faker.string.alphanumeric({ length: { min: 3, max: 64 } }),
        number: faker.number.int({ min: 1, max: 2000 }),
        zip_code: faker.number.int({ min: 1000, max: 99999 }),
    }
}

function randomItem() {
    return {
        name: faker.string.alphanumeric({ length: { min: 1, max: 128 } }),
        price_in_cents: faker.number.int({ min: 1, max: 1_000_000_000 }),
    }
}

function randomOrderItem() {
    return {
        item: randomItem(),
        quantity: faker.number.int({ min: 1, max: 10_000 }),
    }
}

export function randomInvoice(id) {
    return {
        _id: new ObjectId(id),
        items: Array.from({ length: faker.number.int({ min: 1, max: 100 }) }).map(randomOrderItem),
        billing_address: randomAddress(),
        shipping_address: randomAddress(),
        user_id: faker.string.alphanumeric({ length: { min: 10, max: 24 } }),
        tax_rate: 0.15,
        issued_at: Date.now(),
        extra_info: faker.string.alphanumeric({ length: { min: 0, max: 512 } }),
        status: 'OPEN',
        invoice_number: faker.string.alphanumeric({ length: { min: 10, max: 13 } }),
    }
}