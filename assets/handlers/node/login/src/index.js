import { readUserByEmail, readUserByUsername } from './read-user.js';
import { verify } from 'argon2';
import { createSession } from './session.js';

const emailRegex = /(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)/;

/**
 *
 * @param usernameOrEmail {string}
 * @param password {string}
 * @returns {Promise<Record<string, any> | undefined>}
 */
export async function loginWithUsernameOrEmail(usernameOrEmail, password) {
  const user = emailRegex.test(usernameOrEmail)
    ? await readUserByEmail(usernameOrEmail)
    : await readUserByUsername(usernameOrEmail);

  if (user && (await verify(user.password, password))) {
    return await createSession(user.id);
  }

  return undefined;
}
