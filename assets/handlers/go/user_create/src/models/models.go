package models

import (
	"time"
	"user_create/src/hash"

	"github.com/go-playground/validator/v10"
)

type User struct {
	Username     string    `json:"username" validate:"required,min=3,max=64"`
	Email        string    `json:"email" validate:"required,email,min=3,max=64"`
	PasswordHash string    `json:"password" validate:"required,min=32,max=128"`
	CreatedAt    time.Time `json:"created_at"`
}

// Constructor function for creating a new User
func NewUser(userData User) (*User, error) {
	// Validate the user struct
	validate := validator.New()
	err := validate.Struct(userData)
	if err != nil {
		return nil, err
	}

	passwordHash := hash.HashPassword(userData.PasswordHash)

	// Create a new user
	user := &User{
		Username:     userData.Username,
		Email:        userData.Email,
		PasswordHash: passwordHash,
		CreatedAt:    time.Now().UTC(),
	}

	return user, nil
}
