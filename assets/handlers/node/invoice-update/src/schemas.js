import Joi from 'joi';

export const addressSchema = Joi.object({
  first_name: Joi.string().min(2).max(64),
  last_name: Joi.string().min(2).max(64),
  street: Joi.string().min(2).max(128),
  number: Joi.number().min(0).max(10000),
  zip_code: Joi.number().min(1000).max(99999),
  city: Joi.string().min(3).max(64),
  country: Joi.string().min(3).max(64),
});

export const itemSchema = Joi.object({
  price_in_cents: Joi.number().min(0).max(1000000),
  name: Joi.string().min(1).max(128),
});

export const orderItemSchema = Joi.object({
  quantity: Joi.number().min(0).max(10000),
  item: itemSchema,
});

export const invoiceSchema = Joi.object({
  items: Joi.array().items(orderItemSchema).optional(),
  billing_address: addressSchema.optional(),
  shipping_address: addressSchema.optional(),
  user_id: Joi.string().optional(),
  tax_rate: Joi.number().optional(),
  issued_at: Joi.number().optional(),
  extra_info: Joi.string().optional(),
  status: Joi.string().valid('OPEN', 'PAID').optional(),
});
