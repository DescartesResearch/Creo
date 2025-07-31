import Joi from 'joi';

export const sessionSchema = Joi.object({
  user_id: Joi.string(),
  session_id: Joi.string(),
  exp: Joi.number().default(() => Date.now() + 259200000), // == 3 days
});

export const sessionResponseSchema = Joi.object({
  token: Joi.string(),
  exp: Joi.number(),
});
