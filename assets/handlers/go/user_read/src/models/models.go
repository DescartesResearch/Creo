package models

import (
	"time"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

type User struct {
	ID        primitive.ObjectID `json:"id"`
	Username  string             `json:"username" validate:"required,min=3,max=64"`
	Email     string             `json:"email" validate:"required,min=3,max=64"`
	CreatedAt time.Time          `json:"created_at"`
}
