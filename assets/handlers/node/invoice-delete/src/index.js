import { invoiceCollection } from './db.js';
import { ObjectId } from 'mongodb';

/**
 *
 * @param id {number}
 * @returns {Promise<boolean>}
 */
export async function deleteInvoiceById(id) {
  const insertResult = await invoiceCollection.deleteOne({
    _id: new ObjectId(id),
  });

  return insertResult.acknowledged;
}
