import { validate } from './validate.js';
import { userCollection } from './db.js';

/**
 *
 * @param buffer {Buffer}
 * @returns {Promise<string>}
 */
export async function createUser(buffer) {
  const invoice = await validate(buffer);

  const insertResult = await userCollection.insertOne(invoice);

  return insertResult.insertedId.toString();
}
