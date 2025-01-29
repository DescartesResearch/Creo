import { userCollection } from './db.js';

/**
 *
 * @param user {Record<PropertyKey, any>}
 * @returns {Promise<string>}
 */
export async function createUser(user) {
  const insertResult = await userCollection.insertOne(user);

  return insertResult.insertedId.toString();
}
