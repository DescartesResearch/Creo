package read_invoice

import (
	"context"
	"errors"
	"invoice_read/src/db"
	"invoice_read/src/models"
	"invoice_read/src/unmarshal"
	"time"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

// Gets an invoice by ID.
func ReadInvoiceByID(id string) (*models.Invoice, error) {
	// Context for the MongoDB query
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	filter := bson.M{"_id": id}

	var result map[string]any
	err := db.InvoiceCollection.FindOne(ctx, filter).Decode(&result)
	if err != nil {
		if errors.Is(err, mongo.ErrNoDocuments) {
			return nil, nil
		}
		return nil, err
	}

	// Replace MongoDB _id with id
	if val, ok := result["_id"]; ok {
		result["id"] = val
		delete(result, "_id")
	}

	invoice, err := unmarshal.UnmarshalInvoice(result)
	if err != nil {
		return nil, err
	}

	return &invoice, nil
}
