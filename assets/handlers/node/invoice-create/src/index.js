import { validate } from './validate.js';
import { invoiceCollection } from './db.js';

/**
 *
 * @param buffer {Buffer}
 * @returns {Promise<string>}
 */
export async function createInvoice(buffer) {
  const invoice = await validate(buffer);

  const insertResult = await invoiceCollection.insertOne(invoice);

  return insertResult.insertedId.toString();
}
