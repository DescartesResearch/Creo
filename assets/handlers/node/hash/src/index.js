import { hash as argon2 } from 'argon2';

const options = {
  timeCost: 2,
  memoryCost: 6144,
  saltLength: 16,
};

/**
 *
 * @param password {string}
 * @returns {Promise<{hash: string}>}
 */
export async function hashPassword(password) {
  return {
    hash: await argon2(password, options),
  };
}
