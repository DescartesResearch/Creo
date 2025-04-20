package random

import (
	"errors"
	"math/rand"
	"time"
)

// Generates random integers in the range [min, max]
// and sends them to the provided channel. Using a
// goroutine and a Go channel for asynchronous number production.
// See: https://go.dev/ref/spec#Channel_types
func yieldRandomNumber(min, max int, ch chan int, randGen *rand.Rand) {
	for {
		ch <- randGen.Intn(max-min+1) + min
	}
}

// Generates n random numbers between min and max.
func GenerateRandomNumbers(n, min, max int) ([]int, error) {
	if n < 1 {
		return nil, errors.New(`"n" must be greater or equal to 1`)
	}

	// Swap if min > max
	if min > max {
		min, max = max, min
	}

	// Create a random number generator
	randGen := rand.New(rand.NewSource(time.Now().UnixNano()))

	// Create a channel to receive random numbers
	ch := make(chan int)

	// Start the goroutine to generate random numbers
	go yieldRandomNumber(min, max, ch, randGen)

	numbers := make([]int, n)

	// Collect n random numbers from the channel
	for i := range numbers {
		numbers[i] = <-ch
	}

	return numbers, nil
}
