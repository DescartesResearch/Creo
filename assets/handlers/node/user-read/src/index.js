import { userCollection } from './db.js';
import { ObjectId } from 'mongodb';

/**
 *
 * @param id {string}
 * @returns {Promise<any>}
 */
export async function readUserById(id) {
  const user = await userCollection.findOne({ _id: new ObjectId(id) });

  if (!user) {
    return undefined;
  }

  user['id'] = user._id.toString();
  delete user._id;

  return user;
}
