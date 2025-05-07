package update

import (
	"context"
	"user_update/src/db"
	"user_update/src/models"
	"user_update/src/unmarshal"

	"go.mongodb.org/mongo-driver/bson"
)

// Updates the user by ID.
func UpdateUserById(id int, jsonData []byte) (bool, error) {
	userStruct, err := unmarshal.UnmarshalUser(jsonData)
	if err != nil {
		return false, err
	}

	user, _ := models.NewUser(userStruct)

	updateMap := bson.M{}

	// Add fields to the update map if they are not empty (excluding nil or default values)
	if user.Username != "" {
		updateMap["username"] = user.Username
	}
	if user.Email != "" {
		updateMap["email"] = user.Email
	}
	if user.PasswordHash != "" {
		updateMap["password_hash"] = user.PasswordHash
	}

	updateResult, err := db.UserCollection.UpdateOne(
		context.Background(),
		bson.M{"_id": id},
		bson.M{"$set": updateMap},
	)
	if err != nil {
		return false, err
	}

	return updateResult.ModifiedCount > 0, nil
}
