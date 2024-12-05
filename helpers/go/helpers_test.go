package helpers_test

import (
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
	"strings"
	"testing"
)

func TestGetInDirection(t *testing.T) {
	testInput := `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`
	var testInputBytes [][]byte
	for _, line := range strings.Split(testInput, "\n") {
		testInputBytes = append(testInputBytes, []byte(line))
	}

	k, l, e := helpers.GetXInDirection(4, testInputBytes, helpers.Coordinates{X: 5, Y: 0}, helpers.E)
	if string(k) != "XMAS" {
		t.Fatalf("expected XMAS got %v", string(k))
	}
	if e != nil {
		t.Fatalf("Received error")
	}
	if l.X != 8 || l.Y != 0 {
		t.Fatalf("invalid coords")
	}
}
