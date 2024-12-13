package Day04

import (
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
	"strconv"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 4" }

func (ctx *Context) Part1() string {
	grid := helpers.ReadInputAsByteArray(ctx.Path)

	count := 0
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[0]); x++ {
			if word, _, _ := helpers.GetXInDirection(4, grid, helpers.Coordinates{X: x, Y: y}, helpers.NW); string(word) == "XMAS" {
				count++
			}
			if word, _, _ := helpers.GetXInDirection(4, grid, helpers.Coordinates{X: x, Y: y}, helpers.N); string(word) == "XMAS" {
				count++
			}
			if word, _, _ := helpers.GetXInDirection(4, grid, helpers.Coordinates{X: x, Y: y}, helpers.NE); string(word) == "XMAS" {
				count++
			}
			if word, _, _ := helpers.GetXInDirection(4, grid, helpers.Coordinates{X: x, Y: y}, helpers.W); string(word) == "XMAS" {
				count++
			}
			if word, _, _ := helpers.GetXInDirection(4, grid, helpers.Coordinates{X: x, Y: y}, helpers.E); string(word) == "XMAS" {
				count++
			}
			if word, _, _ := helpers.GetXInDirection(4, grid, helpers.Coordinates{X: x, Y: y}, helpers.SW); string(word) == "XMAS" {
				count++
			}
			if word, _, _ := helpers.GetXInDirection(4, grid, helpers.Coordinates{X: x, Y: y}, helpers.S); string(word) == "XMAS" {
				count++
			}
			if word, _, _ := helpers.GetXInDirection(4, grid, helpers.Coordinates{X: x, Y: y}, helpers.SE); string(word) == "XMAS" {
				count++
			}

		}
	}

	return strconv.Itoa(count)

}
func (ctx *Context) Part2() string {
	grid := helpers.ReadInputAsByteArray(ctx.Path)

	count := 0
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[0]); x++ {
			// Check corner neighbors
			if grid[y][x] == 'A' {
				nw, _, _ := helpers.GetInDirection(grid, helpers.Coordinates{X: x, Y: y}, helpers.NW)
				se, _, _ := helpers.GetInDirection(grid, helpers.Coordinates{X: x, Y: y}, helpers.SE)
				ne, _, _ := helpers.GetInDirection(grid, helpers.Coordinates{X: x, Y: y}, helpers.NE)
				sw, _, _ := helpers.GetInDirection(grid, helpers.Coordinates{X: x, Y: y}, helpers.SW)
				if ((nw == 'S' && se == 'M') || (nw == 'M' && se == 'S')) && ((ne == 'S' && sw == 'M') || (ne == 'M' && sw == 'S')) {
					count++
				}
			}
		}
	}

	return strconv.Itoa(count)
}
