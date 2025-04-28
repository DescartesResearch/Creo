package delete_invoice

import (
	"context"
	"fmt"
	"invoice_delete/src/db"
	"time"
)

// Deletes an invoice by its ID.
func DeleteInvoiceByID(id string) (bool, error) {
	// Create the context with a timeout for MongoDB operation
	// timeout can be increased/decreased depending on the needs.
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	deleteResult, err := db.InvoiceCollection.DeleteOne(ctx, map[string]any{"_id": id})
	if err != nil {
		return false, fmt.Errorf("failed to delete invoice: %v", err)
	}

	return deleteResult.DeletedCount > 0, nil
}
