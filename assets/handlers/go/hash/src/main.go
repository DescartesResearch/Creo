package hash

import (
	"crypto/rand"
	"fmt"

	"golang.org/x/crypto/argon2"
)

const (
	TIME_COST   = 1      
	MEMORY_COST = 6144   
	SALT_LEN    = 16    
	KEY_LEN     = 32     
)

// Hashes a password using the Argon2 hash function and returns the hashed string
//
// Arguments:
// - password {string}: The password to be hashed
//
// Returns:
// - map[string]string: A map containing the hashed password
//
// Example:
// hashedPassword := HashPassword("password")
func HashPassword(password string) map[string]string {
	// Generate salt and fill it with crypto/rand to pass to Argon2 IDKey
	salt := make([]byte, SALT_LEN)
  rand.Read(salt)

	// Hash the password using Argon2 IDKey
	// See: https://pkg.go.dev/golang.org/x/crypto/argon2
	hash := argon2.IDKey([]byte(password), salt, TIME_COST, MEMORY_COST, 4, KEY_LEN)

	// Convert hash to string representation
	hashPassword := fmt.Sprintf("%x", hash)

	return map[string]string{"hash": hashPassword}
}
