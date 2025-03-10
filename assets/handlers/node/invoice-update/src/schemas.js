import Joi from 'joi';

export const addressSchema = Joi.object({
  first_name: Joi.string().min(2).max(64),
  last_name: Joi.string().min(2).max(64),
  street: Joi.string().min(2).max(64),
  number: Joi.number().greater(0),
  zip_code: Joi.string(),
  city: Joi.string(),
  country: Joi.string(),
});

export const itemSchema = Joi.object({
  price_in_cents: Joi.number().greater(0),
  name: Joi.string().min(1).max(128),
});

export const orderItemSchema = Joi.object({
  quantity: Joi.number().greater(0),
  item: itemSchema,
});

export const invoiceSchema = Joi.object({
  items: Joi.array().items(orderItemSchema).optional(),
  billing_address: addressSchema.optional(),
  shipping_address: addressSchema.optional(),
  user_id: Joi.string().optional(),
  tax_rate: Joi.number().optional(),
  issued_at: Joi.date().optional(),
  extra_info: Joi.string().optional(),
  status: Joi.string().valid('OPEN', 'PAID').optional(),
});
