package models

import (
	"user_update/src/hash"

	"github.com/go-playground/validator/v10"
)

type User struct {
	Username     string `json:"username" validate:"required,min=3,max=64"`
	Email        string `json:"email" validate:"required,email,min=3,max=64"`
	PasswordHash string `json:"password" validate:"required,min=32,max=128"`
}

// Constructor function for creating and validating a new User
func NewUser(userData User) (*User, error) {
	// Validate the userData
	validate := validator.New()
	err := validate.Struct(userData)
	if err != nil {
		return nil, err
	}

	passwordHash := hash.HashPassword(userData.PasswordHash)

	user := &User{
		Username:     userData.Username,
		Email:        userData.Email,
		PasswordHash: passwordHash,
	}

	return user, nil
}
