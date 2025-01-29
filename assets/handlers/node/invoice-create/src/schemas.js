import Joi from 'joi';

export const addressSchema = Joi.object({
  firstName: Joi.string().min(2).max(64),
  lastName: Joi.string().min(2).max(64),
  street: Joi.string().min(2).max(64),
  number: Joi.number().greater(0),
  zipCode: Joi.string(),
  city: Joi.string(),
  country: Joi.string(),
});

export const itemSchema = Joi.object({
  priceInCent: Joi.number().greater(0),
  name: Joi.string().min(1).max(128),
});

export const orderItemSchema = Joi.object({
  quantity: Joi.number().greater(0),
  item: itemSchema,
});

export const invoiceSchema = Joi.object({
  items: Joi.array().items(orderItemSchema),
  billingAddress: addressSchema,
  shippingAddress: addressSchema,
  userId: Joi.string(),
  taxRate: Joi.number().default(0.15),
  issuedAt: Joi.date().default(Date.now),
  extraInfo: Joi.string().default(''),
  status: Joi.string().valid('OPEN', 'PAID').default('OPEN'),
  invoice_number: Joi.string(),
});
