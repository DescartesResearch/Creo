import { MongoClient } from 'mongodb';

const HOST = process.env['DB_MONGO_HOST'];
const PORT = process.env['DB_MONGO_PORT'] ?? '';
const USER = process.env['DB_MONGO_USER'] ?? '';
const PASSWORD = process.env['DB_MONGO_PASSWORD'] ?? '';

const url = (USER && PASSWORD) ? `mongodb://${USER}:${PASSWORD}@${HOST}:${PORT}`: `mongodb://${HOST}:${PORT}`;

const client = new MongoClient(url);

await client.connect();

const userDb = client.db('user_db');
export const userCollection = userDb.collection('user_collection');