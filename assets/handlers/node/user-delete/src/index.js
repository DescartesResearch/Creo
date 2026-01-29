import { userCollection } from './db.js';
import { ObjectId } from 'mongodb';

/**
 *
 * @param id {string}
 * @returns {Promise<boolean>}
 */
export async function deleteUserById(id) {
  const insertResult = await userCollection.deleteOne({
    _id: new ObjectId(id),
  });

  return insertResult.acknowledged;
}
