package matrix

import (
	log "fmt"
	random "math/rand"
	"testing"
	"time"
)

func TestInvertRandomMatrix(t *testing.T) {
	size := 3
	// Generate the inverse matrix
	invMatrix, err := InvertRandomMatrix(size)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	// Generate the original random matrix inside the function
	// (since InvertRandomMatrix already creates a random matrix)
	originalMatrix := make([][]float64, size)
	randGen := random.New(random.NewSource(time.Now().UnixNano()))
	for i := range size {
		originalMatrix[i] = make([]float64, size)
		for j := 0; j < size; j++ {
			originalMatrix[i][j] = randGen.Float64()
		}
	}

	// Print the original matrix and its inverse
	log.Println("Original Matrix:")
	printMatrix(originalMatrix)

	log.Println("Inverse Matrix:")
	printMatrix(invMatrix)

	// Check that the matrix is size x size
	if len(invMatrix) != size {
		t.Errorf("Expected %d rows, got %d", size, len(invMatrix))
	}
	for i, row := range invMatrix {
		if len(row) != size {
			t.Errorf("Row %d: expected %d columns, got %d", i, size, len(row))
		}
	}
}

// Helper function to print matrices
func printMatrix(matrix [][]float64) {
	for _, row := range matrix {
		log.Println(row)
	}
}
