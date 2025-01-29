import Joi from 'joi';
import { hashPassword } from './hash.js';

export const userSchema = Joi.object({
  username: Joi.string().min(3).max(64),
  email: Joi.string().min(3).max(64),
  password_hash: Joi.binary().min(32).max(128),
  created_at: Joi.date().default(Date.now),
});

export const createUserSchema = Joi.object({
  username: Joi.string().min(3).max(64),
  email: Joi.string().min(3).max(64),
  password_hash: Joi.binary().min(32).max(128),
  created_at: Joi.date().default(Date.now),
});

createUserSchema.external(async (user) => {
  user.password_hash = Buffer.from(await hashPassword(user.password_hash));

  return user;
});
