import Joi from 'joi';

export const addressSchema = Joi.object({
  first_name: Joi.string().min(2).max(64),
  last_name: Joi.string().min(2).max(64),
  street: Joi.string().min(2).max(64),
  number: Joi.number().greater(0),
  zip_code: Joi.number(),
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
  items: Joi.array().items(orderItemSchema),
  billing_address: addressSchema,
  shipping_address: addressSchema,
  user_id: Joi.string(),
  tax_rate: Joi.number().default(0.15),
  issuedAt: Joi.date().default(Date.now),
  extra_info: Joi.string().default(''),
  status: Joi.string().valid('OPEN', 'PAID').default('OPEN'),
  invoice_number: Joi.string(),
});