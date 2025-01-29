import { invoiceCollection } from './db.js';
import { ObjectId } from 'mongodb';
import { validate } from './validate.js';

/**
 *
 * @param id {number}
 * @param buffer {Buffer}
 * @returns {Promise<boolean>}
 */
export async function readInvoiceById(id, buffer) {
  const invoice = await validate(buffer);

  if (invoice) {
    const updateResult = await invoiceCollection.updateOne(
      { _id: new ObjectId(id) },
      {
        $set: invoice,
      },
    );

    return updateResult.acknowledged;
  }

  return true;
}
