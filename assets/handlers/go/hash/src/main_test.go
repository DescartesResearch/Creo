package hash

import (
	"testing"
)

// Tests the HashPassword function.
func TestHashPassword(t *testing.T) {
	password := "password"

	// Call the HashPassword function
	hashedPassword := HashPassword(password)

	hash := hashedPassword["hash"]

	// Check if the hashed password is not the same as the input password
	if hash == password {
		t.Errorf("Password is not hashed correctly. Expected a hashed value.")
	}

	// Check if the hashed password is not empty
	if len(hash) == 0 {
		t.Errorf("Expected a non-empty hashed password, but got an empty string.")
	}
}
