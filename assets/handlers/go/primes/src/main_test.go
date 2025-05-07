package prime

import (
	"fmt"
	"testing"
)

func TestGenerateFirstPrimes(t *testing.T) {
	primes, err := GenerateFirstPrimes(5)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	expected := []int{2, 3, 5, 7, 11}
	for i, val := range expected {
		if primes[i] != val {
			t.Errorf("Expected primes[%d] = %d, got %d", i, val, primes[i])
		}
	}

	fmt.Println("Generated primes:", primes)
}
