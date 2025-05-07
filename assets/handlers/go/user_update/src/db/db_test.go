package db

import (
	"context"
	log "fmt"
	"testing"
	"time"
)

func TestInitMongo(t *testing.T) {
	InitMongo()

	if UserDb == nil {
		t.Errorf("Expected a non-nil database connection, got nil")
	}

	if UserCollection == nil {
		t.Errorf("Expected a non-nil collection, got nil")
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	count, err := UserCollection.CountDocuments(ctx, map[string]any{})
	if err != nil {
		t.Errorf("Failed to count documents: %v", err)
	}

	log.Printf("📦 Total documents in 'login_collection': %d\n", count)

	if count != 0 {
		t.Errorf("Expected 0 documents, but got %d", count)
	}
}
