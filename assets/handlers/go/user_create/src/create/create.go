package create

import (
	"context"
	"user_create/src/db"
	"user_create/src/models"
	"user_create/src/unmarshal"
)

// Creates a new user in Database.
func CreateUser(jsonData []byte) (string, error) {
	// Unmarshal JSON data into a User struct
	userStruct, err := unmarshal.UnmarshalUser(jsonData)
	if err != nil {
		return "", err
	}

	user, _ := models.NewUser(userStruct)

	insertResult, err := db.UserCollection.InsertOne(context.Background(), user)
	if err != nil {
		return "", err
	}

	// Return the inserted ID as string
	return insertResult.InsertedID.(string), nil
}
