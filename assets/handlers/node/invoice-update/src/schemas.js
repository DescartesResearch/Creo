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
  items: Joi.array().items(orderItemSchema).optional(),
  billingAddress: addressSchema.optional(),
  shippingAddress: addressSchema.optional(),
  userId: Joi.string().optional(),
  taxRate: Joi.number().optional(),
  issuedAt: Joi.date().optional(),
  extraInfo: Joi.string().optional(),
  status: Joi.string().valid('OPEN', 'PAID').optional(),
});
