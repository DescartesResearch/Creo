package read

import (
	"context"
	"errors"
	"time"
	"user_read/src/db"
	"user_read/src/models"
	"user_read/src/unmarshal"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

// Gets a user by ID from the Database.
func ReadUserByID(id string) (*models.User, error) {
	// Create a context for the MongoDB query
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	filter := bson.M{"_id": id}

	var result map[string]any
	err := db.UserCollection.FindOne(ctx, filter).Decode(&result)
	if err != nil {
		if errors.Is(err, mongo.ErrNoDocuments) {
			// Not found
			return nil, nil
		}
		return nil, err
	}

	if val, ok := result["_id"]; ok {
		result["id"] = val
		delete(result, "_id")
	}

	// Convert map to models.User
	user, err := unmarshal.UnmarshalUser(result)
	if err != nil {
		return nil, err
	}

	return &user, nil
}
