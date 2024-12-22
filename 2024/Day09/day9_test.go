package Day09

import (
	"testing"
)

func TestDay1(t *testing.T) {
	testCtx := Context{Path: "./testinput"}
	expected := "1928"
	output := testCtx.Part1()
	if expected != output {
		t.Errorf("Expected: %v, got %v\n", expected, output)
	}
}

func TestDay2(t *testing.T) {
	testCtx := Context{Path: "./testinput"}
	expected := "2858"
	output := testCtx.Part2()
	if expected != output {
		t.Errorf("Expected: %v, got %v\n", expected, output)
	}
}
