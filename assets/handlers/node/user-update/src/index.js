import { validate } from './validate.js';
import { userCollection } from './db.js';
import { ObjectId } from 'mongodb';

/**
 *
 * @param id {string}
 * @param buffer {Buffer}
 * @returns {Promise<boolean>}
 */
export async function updateUserById(id, buffer) {
  const user = await validate(buffer);

  if (user) {
    const updateResult = await userCollection.updateOne(
      { _id: id },
      {
        $set: user,
      },
    );
    return updateResult.acknowledged;
  }

  return true;
}
