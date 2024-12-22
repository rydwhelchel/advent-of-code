package Day10

import (
	"testing"
)

const (
	part1Expected = "36"
	part2Expected = "81"
)

func TestDay1(t *testing.T) {
	testCtx := Context{Path: "./testinput"}
	output := testCtx.Part1()
	if part1Expected != output {
		t.Errorf("Expected: %v, got %v\n", part1Expected, output)
	}
}

func TestDay2(t *testing.T) {
	testCtx := Context{Path: "./testinput"}
	output := testCtx.Part2()
	if part2Expected != output {
		t.Errorf("Expected: %v, got %v\n", part2Expected, output)
	}
}
