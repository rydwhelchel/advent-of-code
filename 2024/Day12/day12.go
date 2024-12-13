package Day12

import (
	h "github.com/rydwhelchel/advent-of-code/helpers/go"
	"strconv"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 12" }

func (ctx *Context) Part1() string {
	grid := h.ReadInputAsByteArray(ctx.Path)

	var visited = make(map[h.Coordinates]bool)
	sum := 0
	for y := range grid {
		for x := range grid[y] {
			coord := h.Coordinates{
				X: x,
				Y: y,
			}
			if visited[coord] {
				continue
			}
			area, perimeter := calcRegion(grid, coord, visited)
			sum += area * perimeter
		}
	}

	return strconv.Itoa(sum)
}

func calcRegion(grid [][]byte, coordinates h.Coordinates, visited map[h.Coordinates]bool) (int, int) {
	kind := grid[coordinates.Y][coordinates.X]
	area := 1
	perimeter := 0

	visited[coordinates] = true

	q := []h.Coordinates{coordinates}
	for len(q) > 0 {
		curr := q[0]
		q = q[1:]
		neighbors := h.GetNeighbors(grid, curr)
		for _, c := range neighbors {
			// if it is a different type of plant, it's a side
			if c.Val != kind {
				perimeter++
			} else if !visited[c.Coords] {
				area++
				visited[c.Coords] = true
				q = append(q, c.Coords)
			}
		}
	}
	return area, perimeter
}

func (ctx *Context) Part2() string {
	grid := h.ReadInputAsByteArray(ctx.Path)

	var visited = make(map[h.Coordinates]bool)
	sum := 0
	for y := range grid {
		for x := range grid[y] {
			coord := h.Coordinates{
				X: x,
				Y: y,
			}
			if visited[coord] {
				continue
			}
			area, numSides := calcRegion2(grid, coord, visited)
			sum += area * numSides
		}
	}

	return strconv.Itoa(sum)
}

func calcRegion2(grid [][]byte, coordinates h.Coordinates, visited map[h.Coordinates]bool) (int, int) {
	kind := grid[coordinates.Y][coordinates.X]
	area := 1
	numSides := 0

	visited[coordinates] = true

	q := []h.Coordinates{coordinates}
	for len(q) > 0 {
		curr := q[0]
		q = q[1:]
		neighbors := h.GetNeighbors(grid, curr)
		for _, c := range neighbors {
			if c.Val != kind {
				var newDir h.Direction
				// I'm really beginning to hate my decision to treat directions like an enum.
				switch c.Dir {
				case h.N:
					newDir = h.E
				case h.E:
					newDir = h.S
				case h.S:
					newDir = h.W
				case h.W:
					newDir = h.N
				default:
					// impossible
				}

				adjacentVal, adjacent, _ := h.GetInDirection(grid, curr, newDir)
				diagonalVal, _, _ := h.GetInDirection(grid, adjacent, c.Dir)
				// # of corners == # of numSides, so find a corner
				// we're on the pointy end of a corner || we're on the inside of a corner
				if adjacentVal != kind || diagonalVal == kind {
					numSides++
				}
			} else if !visited[c.Coords] {
				area++
				visited[c.Coords] = true
				q = append(q, c.Coords)
			}
		}
	}
	return area, numSides
}
