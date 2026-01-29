import { MongoClient } from 'mongodb';

const HOST = process.env['DB_MONGO_HOST'];
const PORT = process.env['DB_MONGO_PORT'] ?? '';
const USER = process.env['DB_MONGO_USER'];
const PASSWORD = process.env['DB_MONGO_PASSWORD'];

const url = (USER && PASSWORD) ? `mongodb://${USER}:${PASSWORD}@${HOST}:${PORT}`: `mongodb://${HOST}:${PORT}`;

const client = new MongoClient(url);

await client.connect();

export const invoiceDb = client.db('invoice_db');
export const invoiceCollection = invoiceDb.collection('invoice_collection');
