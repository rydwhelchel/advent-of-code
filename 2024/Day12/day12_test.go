package Day12

import (
	h "github.com/rydwhelchel/advent-of-code/helpers/go"
	"testing"
)

func TestCalcRegion(t *testing.T) {
	tt := []struct {
		name          string
		coordX        int
		coordY        int
		expectedArea  int
		expectedPerim int
	}{
		{
			name:          "R",
			coordX:        0,
			coordY:        0,
			expectedArea:  12,
			expectedPerim: 18,
		},
		{
			name:          "I",
			coordX:        4,
			coordY:        0,
			expectedArea:  4,
			expectedPerim: 8,
		},
	}

	for _, tc := range tt {
		t.Run(tc.name, func(t *testing.T) {
			input := `RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE`
			grid := h.ReadRawAsByteArray(input)

			var visited = make(map[h.Coordinates]bool)
			area, perimeter := calcRegion(grid, h.Coordinates{X: tc.coordX, Y: tc.coordY}, visited)
			if area != tc.expectedArea {
				t.Errorf("Wrong area, expected %v, got %v", tc.expectedArea, area)
			}
			if perimeter != tc.expectedPerim {
				t.Errorf("Wrong perimeter, expected %v, got %v", tc.expectedPerim, perimeter)
			}
		})
	}
}
