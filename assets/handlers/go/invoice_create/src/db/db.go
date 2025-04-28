package db

import (
	"context"
	log "fmt"
	logError "log"
	"os"
	"time"

	env "github.com/joho/godotenv"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var (
	InvoiceDb         *mongo.Database
	InvoiceCollection *mongo.Collection
)

func InitMongo() {
	// Load .env
	err := env.Load("../.env")
	if err != nil {
		logError.Fatalf("Failed to load .env file: %v", err)
	}

	// Read env
	protocol := os.Getenv("DB_MONGO_PROTOCOL")
	user := os.Getenv("DB_MONGO_USER")
	pass := os.Getenv("DB_MONGO_PASSWORD")
	host := os.Getenv("DB_MONGO_HOST")
	params := os.Getenv("DB_MONGO_PARAMS")

	uri := log.Sprintf("%s://%s:%s@%s/?%s", protocol, user, pass, host, params)
	clientOptions := options.Client().ApplyURI(uri)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	client, err := mongo.Connect(ctx, clientOptions)
	if err != nil {
		logError.Fatalf("MongoDB connection error: %v", err)
	}

	if err := client.Ping(ctx, nil); err != nil {
		logError.Fatalf("MongoDB ping failed: %v", err)
	}

	log.Println("✅ Connected to MongoDB!")

	InvoiceDb = client.Database("invoice_db")
	InvoiceCollection = InvoiceDb.Collection("invoice_collection")
}
