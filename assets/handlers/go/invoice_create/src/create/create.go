package create

import (
	"context"
	"invoice_create/src/db"
	"invoice_create/src/models"
	"invoice_create/src/unmarshal"
)

// Creates invoice for the given invoice data.
func CreateInvoice(jsonData []byte) (string, error) {
	invoiceStruct, err := unmarshal.UnmarshalInvoice(jsonData)
	if err != nil {
		return "", err
	}

	invoice, err := models.NewInvoice(invoiceStruct)
	if err != nil {
		return "", err
	}

	insertResult, err := db.InvoiceCollection.InsertOne(context.Background(), invoice)
	if err != nil {
		return "", err
	}

	return insertResult.InsertedID.(string), nil
}
