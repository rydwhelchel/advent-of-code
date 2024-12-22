package Day10

import (
	h "github.com/rydwhelchel/advent-of-code/helpers/go"
	im "image"
	"strconv"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 10" }

type Slope struct {
	trailhead im.Point
	location  im.Point
}

func (ctx *Context) Part1() string {
	grid := h.ReadInputAsByteArray(ctx.Path)

	trailheads := createTrailheadsMap(grid)

	sum := 0
	for _, v := range trailheads {
		// for every unique summit we can reach from a trailhead
		for range v {
			sum++
		}
	}

	return strconv.Itoa(sum)
}

func (ctx *Context) Part2() string {
	grid := h.ReadInputAsByteArray(ctx.Path)

	trailheads := createTrailheadsMap(grid)

	sum := 0
	for _, v := range trailheads {
		// for every unique WAY we can reach a summit, accidentally did part2 first :)
		for _, u := range v {
			sum += u
		}
	}

	return strconv.Itoa(sum)
}

func createTrailheadsMap(grid [][]byte) map[im.Point]map[im.Point]int {
	// Initialize the queue with all the trailheads
	var readQueue []Slope
	for y := range grid {
		for x := range grid[y] {
			if grid[y][x] == '0' {
				readQueue = append(readQueue, Slope{im.Point{X: x, Y: y}, im.Point{X: x, Y: y}})
			}
		}
	}

	trailheads := make(map[im.Point]map[im.Point]int)
	// process readQueue, pushing next vals onto end of queue, when we reach 9, ++ to val of trailheads
	for len(readQueue) > 0 {
		curr := readQueue[0]
		x, y := curr.location.X, curr.location.Y
		readQueue = readQueue[1:]
		if grid[y][x] == '9' {
			if _, ok := trailheads[curr.trailhead]; ok {
				// going to ignore the amount of ways we can reach a summit
				trailheads[curr.trailhead][curr.location]++
			} else {
				trailheads[curr.trailhead] = make(map[im.Point]int)
				trailheads[curr.trailhead][curr.location]++
			}
			continue
		}

		currVal, _ := strconv.Atoi(string(grid[y][x]))
		nextVal := strconv.Itoa(currVal + 1)
		nn := h.GetPointNeighbors(grid, curr.location)

		for _, n := range nn {
			if string(grid[n.Y][n.X]) == nextVal {
				readQueue = append(readQueue, Slope{curr.trailhead, n})
			}
		}
	}
	return trailheads
}
