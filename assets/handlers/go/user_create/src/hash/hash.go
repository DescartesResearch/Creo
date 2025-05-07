package hash

import (
	runtime "runtime"

	argon2 "github.com/alexedwards/argon2id"
)

const (
	TIME_COST   = 1
	MEMORY_COST = 6144
	SALT_LEN    = 16
	KEY_LEN     = 32
)

// Hashes a password using the Argon2id hash function and returns the hashed string
//
// Arguments:
// - password {string}: The password to be hashed
//
// Returns:
// - string: A string containing the hashed password
//
// Example:
// hashedPassword := HashPassword("password")
func HashPassword(password string) string {
	// Define custom Argon2id parameters
	params := &argon2.Params{
		Memory:      MEMORY_COST * 1024,
		Iterations:  TIME_COST,
		Parallelism: uint8(runtime.NumCPU()),
		SaltLength:  SALT_LEN,
		KeyLength:   KEY_LEN,
	}

	hashPassword, err := argon2.CreateHash(password, params)
	if err != nil {
		panic(err)
	}

	return hashPassword
}
