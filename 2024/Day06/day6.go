package Day06

import (
	"errors"
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
	"slices"
	"strconv"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 6" }

func (ctx *Context) Part1() string {
	grid := helpers.GetByteGrid(helpers.ReadInputAsLines(ctx.Path))

	guard, err := getGuardLoc(grid)
	if err != nil {
		panic(err)
	}
	dir := helpers.N
	visited := make(map[helpers.Coordinates]int)
	visited[guard]++
	for {
		val, nextLoc, err := helpers.GetInDirection(grid, guard, dir)
		if err != nil && err.Error() == helpers.OFFGRID {
			return strconv.Itoa(len(visited))
		}
		if val != '#' {
			guard = nextLoc
			visited[guard]++
		} else {
			dir = helpers.TurnRight(dir)
		}
	}
}

func getGuardLoc(grid [][]byte) (helpers.Coordinates, error) {
	for y := range grid {
		for x, j := range grid[y] {
			if j == '^' {
				return helpers.Coordinates{X: x, Y: y}, nil
			}
		}
	}
	return helpers.Coordinates{X: -1, Y: -1}, errors.New("no guard :(")
}

func (ctx *Context) Part2() string {
	grid := helpers.GetByteGrid(helpers.ReadInputAsLines(ctx.Path))

	guard, err := getGuardLoc(grid)
	if err != nil {
		panic(err)
	}
	dir := helpers.N
	visited := make(map[helpers.Coordinates]int)
	visited[guard]++
	for {
		val, nextLoc, err := helpers.GetInDirection(grid, guard, dir)
		if err != nil && err.Error() == helpers.OFFGRID {
			break
		}
		if val != '#' {
			guard = nextLoc
			visited[guard]++
		} else {
			dir = helpers.TurnRight(dir)
		}
	}

	type entry struct {
		loc helpers.Coordinates
		dir helpers.Direction
	}
	checkLoop := func(obst helpers.Coordinates) bool {
		g, err := getGuardLoc(grid)
		if err != nil {
			panic(err)
		}
		var newGrid [][]byte
		for _, line := range grid {
			newGrid = append(newGrid, slices.Clone(line))
		}
		newGrid[obst.Y][obst.X] = '#'
		dir := helpers.N
		visited := make(map[entry]int)
		visited[entry{g, dir}]++
		for {
			val, nextLoc, err := helpers.GetInDirection(newGrid, g, dir)
			if err != nil && err.Error() == helpers.OFFGRID {
				break
			}
			if val != '#' {
				g = nextLoc
				ent := entry{g, dir}
				if _, ok := visited[ent]; ok {
					return true
				}
				visited[ent]++
			} else {
				dir = helpers.TurnRight(dir)
			}
		}
		return false
	}

	var inputs = make(chan helpers.Coordinates)
	var results = make(chan bool, len(visited))
	sum := 0

	spawnWorker := func(inputs <-chan helpers.Coordinates, results chan<- bool) {
		for input := range inputs {
			results <- checkLoop(input)
		}
	}

	for i := 0; i < 800; i++ {
		go spawnWorker(inputs, results)
	}

	for poss, _ := range visited {
		inputs <- poss
	}
	close(inputs)
	for i := 0; i < len(visited); i++ {
		if <-results {
			sum++
		}
	}
	return strconv.Itoa(sum)
}

// TODO: Come back and find the solution using turn points, fairly certain it's possible but.. not this way
//func (ctx *Context) Part2() string {
//	grid := helpers.GetByteGrid(helpers.ReadInputAsLines(ctx.Path))
//
//	guard, err := getGuardLoc(grid)
//	if err != nil {
//		panic(err)
//	}
//	dir := helpers.N
//	var nTurnPoints []helpers.Coordinates
//	var eTurnPoints []helpers.Coordinates
//	var sTurnPoints []helpers.Coordinates
//	var wTurnPoints []helpers.Coordinates
//	var possibleLocs []helpers.Coordinates
//	for {
//		val, nextLoc, err := helpers.GetInDirection(grid, guard, dir)
//		if err != nil && err.Error() == helpers.OFFGRID {
//			break
//		}
//		if val != '#' {
//			guard = nextLoc
//		} else {
//			switch dir {
//			case helpers.N:
//				nTurnPoints = append(nTurnPoints, guard)
//			case helpers.E:
//				eTurnPoints = append(eTurnPoints, guard)
//			case helpers.S:
//				sTurnPoints = append(sTurnPoints, guard)
//			case helpers.W:
//				wTurnPoints = append(wTurnPoints, guard)
//			default:
//				panic("WRONG DIR WRONG DIR")
//			}
//			dir = helpers.TurnRight(dir)
//		}
//	}
//	// reset
//	guard, err = getGuardLoc(grid)
//	if err != nil {
//		panic(err)
//	}
//	dir = helpers.N
//	// ~~the ole 2 passes trick eh?~~
//	for {
//		val, nextLoc, err := helpers.GetInDirection(grid, guard, dir)
//		if err != nil && err.Error() == helpers.OFFGRID {
//			break
//		}
//		if val != '#' {
//			guard = nextLoc
//			checkTurnpoints := func(turnPoints []helpers.Coordinates) {
//				for _, loc := range turnPoints {
//					if loc.Intersects(guard) {
//						_, obst, err := helpers.GetInDirection(grid, guard, dir)
//						if err == nil {
//							if !slices.Contains(possibleLocs, obst) {
//								possibleLocs = append(possibleLocs, obst)
//							}
//						}
//					}
//				}
//			}
//			switch dir {
//			case helpers.N:
//				checkTurnpoints(eTurnPoints)
//			case helpers.E:
//				checkTurnpoints(sTurnPoints)
//			case helpers.S:
//				checkTurnpoints(wTurnPoints)
//			case helpers.W:
//				checkTurnpoints(nTurnPoints)
//			default:
//				panic("WRONG DIR WRONG DIR")
//			}
//		} else {
//			dir = helpers.TurnRight(dir)
//		}
//	}
//
//	type entry struct {
//		loc helpers.Coordinates
//		dir helpers.Direction
//	}
//	var definitePositions []helpers.Coordinates
//	// the ole 3 passes trick eh?
//	for _, poss := range possibleLocs {
//		var newGrid [][]byte
//		for _, l := range grid {
//			newGrid = append(newGrid, slices.Clone(l))
//		}
//		newGrid[poss.Y][poss.X] = '#'
//		// reset
//		guard, err = getGuardLoc(grid)
//		if err != nil {
//			panic(err)
//		}
//		visited := make(map[entry]int)
//		visited[entry{guard, dir}]++
//		dir = helpers.N
//		for {
//			val, nextLoc, err := helpers.GetInDirection(newGrid, guard, dir)
//			if err != nil && err.Error() == helpers.OFFGRID {
//				break
//			}
//			if val != '#' {
//				guard = nextLoc
//				if _, ok := visited[entry{guard, dir}]; ok {
//					definitePositions = append(definitePositions, poss)
//					break
//				}
//				visited[entry{guard, dir}]++
//			} else {
//				dir = helpers.TurnRight(dir)
//			}
//		}
//	}
//	return strconv.Itoa(len(definitePositions))
//}
