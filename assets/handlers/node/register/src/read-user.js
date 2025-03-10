import { userCollection } from './db.js';

/**
 *
 * @param key {string}
 * @param value {string}
 * @returns {Promise<any | undefined>}
 */
async function readUserByKey(key, value) {
  const user = await userCollection.findOne({ key: value });

  if (!user) {
    return undefined;
  }

  user['id'] = user._id.toString();
  delete user._id;

  return user;
}

/**
 *
 * @param name {string}
 * @returns {Promise<any>}
 */
export async function readUserByUsername(name) {
  return readUserByKey('username', name);
}

/**
 *
 * @param email {string}
 * @returns {Promise<any>}
 */
export async function readUserByEmail(email) {
  return readUserByKey('email', email);
}
