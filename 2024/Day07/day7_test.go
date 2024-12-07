package Day07

import (
	"slices"
	"testing"
)

func TestGetNextOperators(t *testing.T) {
	tcs := []struct {
		name   string
		input  []int
		output []int
	}{
		{
			"110",
			[]int{1, 1, 0},
			[]int{1, 1, 1},
		},
		{
			"101",
			[]int{1, 0, 1},
			[]int{1, 1, 0},
		},
		{
			"100",
			[]int{1, 0, 0},
			[]int{1, 0, 1},
		},
		{
			"111",
			[]int{1, 1, 1},
			[]int{},
		},
	}

	for _, tc := range tcs {
		t.Run(tc.name, func(t *testing.T) {
			output := getNextOperators(slices.Clone(tc.input))
			if !slices.Equal(output, tc.output) {
				t.Errorf("Failed to convert %+v to %+v, got %+v\n", tc.input, tc.output, output)
			}
		})
	}
}

func TestPossibleCalibration(t *testing.T) {
	tcs := []struct {
		name   string
		input  calibration
		output bool
	}{
		{
			"happy path",
			calibration{1, []int{1, 1}},
			true,
		},
		{
			"len 1",
			calibration{1, []int{1}},
			true,
		},
		{
			"falsey",
			calibration{3, []int{1, 1}},
			false,
		},
		{
			"longy",
			calibration{10, []int{5, 2, 1}},
			true,
		},
	}

	for _, tc := range tcs {
		t.Run(tc.name, func(t *testing.T) {
			output := tc.input.possibleCalibration()
			if output != tc.output {
				t.Errorf("Did not get expected from %+v. Expected %v, got %v", tc.input, tc.output, output)
			}
		})
	}
}
