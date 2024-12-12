package Day11

import (
	"strings"
	"testing"
)

func TestCalcResult(t *testing.T) {
	out1, out2 := calcResult("1234")
	if out1 != "12" {
		t.Errorf("Not 12, got %v\n", out1)
	}
	if out2 != "34" {
		t.Errorf("Not 34, got %v\n", out2)
	}
}

func TestPart1(t *testing.T) {
	input := "125 17"

	vals := strings.Split(input, " ")

	current := make(map[string]int)

	for _, val := range vals {
		current[val]++
	}

	expected := []int{3, 4, 5, 9, 13, 22}

	for i := range 6 {
		next := make(map[string]int)
		for k, v := range current {
			o1, o2 := calcResult(k)
			next[o1] += v
			if o2 != "" {
				next[o2] += v
			}
		}
		current = next
		sum := 0
		for _, v := range current {
			sum += v
		}
		if sum != expected[i] {
			t.Errorf("Sum incorrect, expected: %v, got: %v", expected[i], sum)
		}
	}

}
