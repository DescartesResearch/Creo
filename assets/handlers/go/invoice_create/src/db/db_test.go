package db

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestInitMongo(t *testing.T) {
	InitMongo()

	// Verify if the connection was established
	if InvoiceDb == nil {
		t.Errorf("Expected a non-nil database connection, got nil")
	}

	if InvoiceCollection == nil {
		t.Errorf("Expected a non-nil collection, got nil")
	}

	// Additional check: Count documents in the invoice collection (assuming it's empty)
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	count, err := InvoiceCollection.CountDocuments(ctx, map[string]any{})
	if err != nil {
		t.Errorf("Failed to count documents: %v", err)
	}

	// Print the count for confirmation
	fmt.Printf("📦 Total documents in 'invoice_collection': %d\n", count)

	// Assuming an empty collection, the count should be 0
	if count != 0 {
		t.Errorf("Expected 0 documents, but got %d", count)
	}
}
