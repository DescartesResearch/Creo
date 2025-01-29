import Joi from 'joi';

export const userSchema = Joi.object({
  id: Joi.string(),
  username: Joi.string().min(3).max(64),
  email: Joi.string().min(3).max(64),
  password_hash: Joi.binary().min(32).max(128),
  created_at: Joi.date().default(Date.now),
});

export const sessionSchema = Joi.object({
  user_id: Joi.string(),
  session_id: Joi.string(),
  exp: Joi.date().default(() => Date.now() + 259200000), // == 3 days
});

export const sessionResponseSchema = Joi.object({
  token: Joi.string(),
  exp: Joi.date(),
});
