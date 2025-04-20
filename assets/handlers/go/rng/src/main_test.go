package random

import (
	"fmt"
	"testing"
)

func TestGenerateRandomNumbers(t *testing.T) {
	// Simple valid test case
	n, min, max := 5, 1, 10
	numbers, err := GenerateRandomNumbers(n, min, max)

	// Check if there is no error
	if err != nil {
		t.Errorf("Expected no error, but got %v", err)
	}

	// Check if we got the correct number of random numbers
	if len(numbers) != n {
		t.Errorf("Expected %d numbers, got %d", n, len(numbers))
	}

	// Check if all numbers are within the range [min, max]
	for _, num := range numbers {
		if num < min || num > max {
			t.Errorf("Generated number %d is out of range [%d, %d]", num, min, max)
		}
	}

		// Print the generated numbers
		fmt.Println("Random generated numbers:", numbers)
}
