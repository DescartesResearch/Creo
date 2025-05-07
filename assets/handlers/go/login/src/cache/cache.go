package cache

import (
	random "crypto/rand"
	"encoding/base64"
	"sync"

	"login/src/models"
)

// Used for storing and retrieving session data.
// sync is used so that multiple goroutines can read the cache concurrently
type SessionRepository struct {
	cache map[string]models.SessionData
	mu    sync.RWMutex
}

// Ensures that repo is only initialized once. (Singleton behaviour)
var (
	repo     *SessionRepository
	initOnce sync.Once
)

// Provides a singleton instance of SessionRepository
func GetSessionRepository() *SessionRepository {
	initOnce.Do(func() {
		repo = &SessionRepository{
			cache: make(map[string]models.SessionData),
		}
	})
	return repo
}

// Creates a new session for a user with a unique session ID.
//
// The receiver `(s *SessionRepository)` specifies that the method
// operates on an instance of the `SessionRepository` type
func (s *SessionRepository) SetNewSession(userID string) models.SessionResponse {
	sessionID := generateSessionID()

	// Use NewSessionData from the models package to create session data with expiration set to 3 days
	sessionData := models.NewSessionData(userID, sessionID)

	// locks the cache map so that only one goroutine can modify it at a time which prevents race conditions.
	// After modifying the cache, unlock is called to release the lock.
	s.mu.Lock()
	s.cache[sessionID] = *sessionData
	s.mu.Unlock()

	return models.SessionResponse{
		Token: sessionID,
		Exp:   sessionData.Exp,
	}
}

// Creates a base64 session id
func generateSessionID() string {
	bytes := make([]byte, 24)
	if _, err := random.Read(bytes); err != nil {
		panic("Failed to generate session ID: " + err.Error())
	}
	return base64.URLEncoding.EncodeToString(bytes)
}
