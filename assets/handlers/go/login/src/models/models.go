package models

import (
	"time"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

type User struct {
	ID           primitive.ObjectID `bson:"_id,omitempty" json:"id"`
	Username     string             `bson:"username" json:"username" `
	Email        string             `bson:"email" json:"email"`
	PasswordHash string             `bson:"password_hash" json:"password"`
	CreatedAt    time.Time          `bson:"created_at" json:"created_at"`
}

type SessionData struct {
	UserID    string    `bson:"user_id" json:"user_id"`
	SessionID string    `bson:"session_id" json:"session_id"`
	Exp       time.Time `bson:"exp" json:"exp"`
}

// Creates a new session data and sets expiration to 3 days from now.
func NewSessionData(userID, sessionID string) *SessionData {
	expirationTime := time.Now().Add(3 * 24 * time.Hour)
	return &SessionData{
		UserID:    userID,
		SessionID: sessionID,
		Exp:       expirationTime,
	}
}

type SessionResponse struct {
	Token string    `json:"token"`
	Exp   time.Time `json:"exp"`
}
