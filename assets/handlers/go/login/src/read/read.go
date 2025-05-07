package read

import (
	"context"
	logError "log"
	"login/src/db"
	"login/src/models"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

// Finds a user by a specified key (e.g username or email).
func readUserByKey(key string, value string) (*models.User, error) {
	var user models.User
	filter := bson.M{key: value}

	err := db.UserCollection.FindOne(context.Background(), filter).Decode(&user)
	if err == mongo.ErrNoDocuments {
		// Return nil if no document found
		return nil, nil
	}
	if err != nil {
		logError.Println("Error reading user by key:", err)
		return nil, err
	}

	// Return the user found
	return &user, nil
}

// Returns a user by their username
func ReadUserByUsername(username string) (*models.User, error) {
	return readUserByKey("username", username)
}

// Returns a user by their email
func ReadUserByEmail(email string) (*models.User, error) {
	return readUserByKey("email", email)
}
