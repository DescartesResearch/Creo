package login

import (
	"login/src/cache"
	"login/src/models"
	"login/src/read"
	"regexp"

	argon2 "github.com/alexedwards/argon2id"
)

// Email regex to check if input is email
var emailRegex = regexp.MustCompile(`^[a-zA-Z0-9_.+\-]+@[a-zA-Z0-9\-]+\.[a-zA-Z0-9.\-]+$`)

// Login and returns session if successful
func LoginWithUsernameOrEmail(usernameOrEmail, password string) (*models.SessionResponse, error) {
	var user *models.User
	var err error

	if emailRegex.MatchString(usernameOrEmail) {
		user, err = read.ReadUserByEmail(usernameOrEmail)
	} else {
		user, err = read.ReadUserByUsername(usernameOrEmail)
	}

	if err != nil {
		return nil, err
	}

	if user == nil {
		return nil, nil
	}

	ok, err := argon2.ComparePasswordAndHash(password, user.PasswordHash)
	if err != nil || !ok {
		return nil, nil
	}

	// Create and return session
	repo := cache.GetSessionRepository()
	session := repo.SetNewSession(user.ID.Hex())
	return &session, nil
}
