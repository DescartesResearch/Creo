import { createUserSchema } from './schemas.js';

/**
 *
 * @param buffer {Buffer}
 * @returns {Promise<any>}
 */
export async function validate(buffer) {
  const json = JSON.parse(buffer.toString('utf-8'));

  return await createUserSchema.validateAsync(json);
}
