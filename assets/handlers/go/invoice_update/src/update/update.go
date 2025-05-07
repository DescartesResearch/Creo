package update

import (
	"context"
	"invoice_update/src/db"
	"invoice_update/src/unmarshal"

	"go.mongodb.org/mongo-driver/bson"
)

// Updates an invoice by ID
func UpdateInvoice(id int, jsonData []byte) (bool, error) {
	invoice, err := unmarshal.UnmarshalInvoice(jsonData)
	if err != nil {
		return false, err
	}

	// Create an update map
	updateMap := bson.M{}

	if invoice.Items != nil {
		updateMap["items"] = invoice.Items
	}
	if invoice.BillingAddress != nil {
		updateMap["billing_address"] = invoice.BillingAddress
	}
	if invoice.ShippingAddress != nil {
		updateMap["shipping_address"] = invoice.ShippingAddress
	}
	if invoice.TaxRate != nil {
		updateMap["tax_rate"] = invoice.TaxRate
	}
	if invoice.ExtraInfo != nil {
		updateMap["extra_info"] = invoice.ExtraInfo
	}
	if invoice.Status != nil {
		updateMap["status"] = invoice.Status
	}

	// Update the invoice in db
	updateResult, err := db.InvoiceCollection.UpdateOne(
		context.Background(),
		bson.M{"_id": id},
		bson.M{"$set": updateMap},
	)
	if err != nil {
		return false, err
	}

	return updateResult.ModifiedCount > 0, nil
}
