package Day17

import "testing"

const (
	part1Expected = "4,6,3,5,6,3,5,2,1,0"
	part2Expected = "117440"
)

func TestPart1(t *testing.T) {
	testCtx := Context{Path: "./testinput"}
	output := testCtx.Part1()
	if part1Expected != output {
		t.Errorf("Expected: %v, got %v\n", part1Expected, output)
	}
}

func TestPart2(t *testing.T) {
	testCtx := Context{Path: "./testinput"}
	output := testCtx.Part2()
	if part1Expected != output {
		t.Errorf("Expected: %v, got %v\n", part1Expected, output)
	}
}
