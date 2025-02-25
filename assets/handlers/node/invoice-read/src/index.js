import { invoiceCollection } from './db.js';
import { ObjectId } from 'mongodb';

/**
 *
 * @param id {number}
 * @returns {Promise<Record<string, any> | undefined>}
 */
export async function readInvoiceById(id) {
  const invoice = await invoiceCollection.findOne({ _id: new ObjectId(id) });

  if (!invoice) {
    return undefined;
  }

  invoice['id'] = invoice._id.toString();
  delete invoice._id;

  return invoice;
}
