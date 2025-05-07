package matrix

import (
	"errors"
	random "math/rand"
	"time"

	matrix "gonum.org/v1/gonum/mat"
)

// Generates a random square matrix of the given size and returns its inverse.
func InvertRandomMatrix(size int) ([][]float64, error) {
	if size < 1 {
		return nil, errors.New("matrix size must be greater or equal to 1")
	}

	// Use a local, non-global rand generator
	r := random.New(random.NewSource(time.Now().UnixNano()))

	// Create random matrix data
	data := make([]float64, size*size)
	for i := range data {
		data[i] = r.Float64()
	}

	// Create matrix from data
	A := matrix.NewDense(size, size, data)

	// Compute the inverse
	var inv matrix.Dense
	err := inv.Inverse(A)
	if err != nil {
		return nil, errors.New("matrix is singular and cannot be inverted")
	}

	// Convert result to [][]float64
	result := make([][]float64, size)
	for i := range size {
		result[i] = make([]float64, size)
		for j := range size {
			result[i][j] = inv.At(i, j)
		}
	}

	return result, nil
}
