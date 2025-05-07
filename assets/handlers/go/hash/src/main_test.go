package hash

import (
	"testing"

	log "fmt"
)

// Tests the HashPassword function.
func TestHashPassword(t *testing.T) {
	password := "password"

	hashedPassword := HashPassword(password)

	log.Println(hashedPassword)
	hash := hashedPassword["hash"]

	if hash == password {
		t.Errorf("Password is not hashed correctly. Expected a hashed value.")
	}

	// Check if the hashed password is not empty
	if len(hash) == 0 {
		t.Errorf("Expected a non-empty hashed password, but got an empty string.")
	}
}
