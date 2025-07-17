import Joi from 'joi';
import { hashPassword } from './hash.js';

export const createUserSchema = Joi.object({
  username: Joi.string().min(3).max(64),
  email: Joi.string().min(3).max(64),
  password: Joi.string().min(6).max(48),
  created_at: Joi.number().default(Date.now),
}).external(async (user) => {
  user.password = Buffer.from(await hashPassword(user.password));

  return user;
});
