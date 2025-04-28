package login

import (
	log "fmt"
	"testing"

	"login/src/cache"
	"login/src/db"
	"login/src/read"

	"github.com/alexedwards/argon2id"
	"github.com/stretchr/testify/assert"
)

// TestLoginWithUsernameOrEmail tests login for a single user
func TestLoginWithUsernameOrEmail(t *testing.T) {
	usernameOrEmail := "darij"
	correctPassword := "password"
	incorrectPassword := "wrongpassword"

	// Initialize the DB connection
	db.InitMongo()

	// Retrieve the user from the database by username or email
	user, err := read.ReadUserByUsername(usernameOrEmail)
	if err != nil {
		t.Fatalf("Error retrieving user: %v", err)
	}

	if user == nil {
		t.Fatalf("User not found in the database")
	}

	log.Println(user)

	// Validate the correct password against the hashed password
	t.Run("CorrectPassword", func(t *testing.T) {
		// Check if the password matches the hash
		valid, err := argon2id.ComparePasswordAndHash(correctPassword, user.PasswordHash)
		if err != nil {
			t.Fatalf("Error verifying password: %v", err)
		}

		// If the password is correct, proceed with the login
		if valid {
			// Generate a session ID for the user
			sessionRepo := cache.GetSessionRepository()
			sessionResponse := sessionRepo.SetNewSession(user.ID.Hex())

			assert.NotNil(t, sessionResponse)
			assert.NotEmpty(t, sessionResponse.Token)
		} else {
			t.Fatalf("Password verification failed")
		}
	})

	t.Run("IncorrectPassword", func(t *testing.T) {
		// Attempt login with incorrect password
		valid, err := argon2id.ComparePasswordAndHash(incorrectPassword, user.PasswordHash)
		if err != nil {
			t.Fatalf("Error verifying password: %v", err)
		}

		// Assert that the password is incorrect, hence login should fail
		assert.False(t, valid, "Password should be invalid")
	})
}
