package db

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestInitMongo(t *testing.T) {
	InitMongo()

	if InvoiceDb == nil {
		t.Errorf("Expected a non-nil database connection, got nil")
	}

	if InvoiceCollection == nil {
		t.Errorf("Expected a non-nil collection, got nil")
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	count, err := InvoiceCollection.CountDocuments(ctx, map[string]any{})
	if err != nil {
		t.Errorf("Failed to count documents: %v", err)
	}

	fmt.Printf("📦 Total documents in 'invoice_collection': %d\n", count)

	if count != 0 {
		t.Errorf("Expected 0 documents, but got %d", count)
	}
}
