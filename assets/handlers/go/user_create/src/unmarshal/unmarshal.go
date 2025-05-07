package unmarshal

import (
	"encoding/json"
	"user_create/src/models"
)

// Returns a User struct
func UnmarshalUser(jsonData []byte) (models.User, error) {
	var user models.User

	err := json.Unmarshal(jsonData, &user)
	if err != nil {
		return models.User{}, err
	}

	return user, nil
}
