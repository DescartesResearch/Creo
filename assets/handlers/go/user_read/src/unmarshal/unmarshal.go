package unmarshal

import (
	"encoding/json"

	"user_read/src/models"
)

// Converts the provided map into a User struct
func UnmarshalUser(data map[string]any) (models.User, error) {
	// First successfully converts to json
	jsonData, err := json.Marshal(data)
	if err != nil {
		return models.User{}, err
	}

	// Now unmarshal the JSON into the User struct
	var user models.User
	err = json.Unmarshal(jsonData, &user)
	if err != nil {
		return models.User{}, err
	}

	return user, nil
}
