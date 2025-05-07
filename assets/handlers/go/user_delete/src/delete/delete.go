package delete

import (
	"context"
	"time"
	"user_delete/src/db"

	"go.mongodb.org/mongo-driver/bson"
)

// Deletes a user with the given ID
func DeleteUserByID(id int) (bool, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	filter := bson.M{"_id": id}

	deleteResult, err := db.UserCollection.DeleteOne(ctx, filter)
	if err != nil {
		return false, err
	}

	return deleteResult.DeletedCount > 0, nil
}
