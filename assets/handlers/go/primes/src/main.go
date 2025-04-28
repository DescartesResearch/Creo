package prime

import (
	"errors"
)

// Sends an infinite stream of prime numbers through a channel.
func generatePrimes() <-chan int {
	ch := make(chan int)
	go func() {
		// map of composites to list of primes
		D := make(map[int][]int)
		// start from 2 to find primes
		q := 2

		for {
			// Go automatically returns 2 values when accessing a map.
			// 1 is the actual value and 2 is the bool property indicating wether the value was found.
			// In this syntax we are declaring inline variables to limit var scope to this if condition.
			if _, found := D[q]; !found {
				// q is a new prime
				// save it in the channel
				ch <- q
				// q*q is non prime. Ex: 3 is prime but 3*3=9 wont be prime
				D[q*q] = []int{q}
			} else {
				// q is not a prime
				for _, p := range D[q] {
					next := p + q
					D[next] = append(D[next], p)
				}
				delete(D, q)
			}
			q++
		}
	}()
	return ch
}

// Returns the first n prime numbers.
func GenerateFirstPrimes(n int) ([]int, error) {
	if n < 1 {
		return nil, errors.New(`"n" must be greater or equal to 1`)
	}
	ch := generatePrimes()
	primes := make([]int, n)
	for i := range n {
		primes[i] = <-ch
	}
	return primes, nil
}
