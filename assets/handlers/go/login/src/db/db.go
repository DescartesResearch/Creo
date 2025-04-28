package db

import (
	"context"
	log "fmt"
	logError "log"
	"os"
	"time"

	"github.com/joho/godotenv"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var (
	UserDb         *mongo.Database
	UserCollection *mongo.Collection
)

func InitMongo() {
	err := godotenv.Load("../.env")
	if err != nil {
		logError.Fatalf("Failed to load .env file: %v", err)
	}

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

	// Assign exported variables
	UserDb = client.Database("login_db")
	UserCollection = UserDb.Collection("login_collection")

	// Create indexes
	indexes := []mongo.IndexModel{
		{Keys: map[string]any{"username": 1}},
		{Keys: map[string]any{"email": 1}},
	}

	_, err = UserCollection.Indexes().CreateMany(ctx, indexes)
	if err != nil {
		logError.Fatalf("Creating indexes failed: %v", err)
	}

	log.Println("✅ Indexes created!")
}
