import { validate } from './validate.js';
import { readUserByEmail, readUserByUsername } from './read-user.js';
import { createUser } from './create-user.js';

/**
 *
 * @param buffer {Buffer}
 * @returns {Promise<string | undefined>}
 */
export async function registerUser(buffer) {
  const user = await validate(buffer);

  if (await readUserByEmail(user.email)) {
    return undefined;
  }

  if (await readUserByUsername(user.username)) {
    return undefined;
  }

  return await createUser(user);
}
