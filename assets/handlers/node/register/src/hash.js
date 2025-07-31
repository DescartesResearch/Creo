import { hash } from 'argon2';

const options = {
  timeCost: 2,
  memoryCost: 6144,
  saltLength: 16,
};

export async function hashPassword(password) {
  return await hash(password, options);
}
