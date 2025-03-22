package helpers

import (
	"errors"
	"fmt"
	im "image"
	"log"
	"os"
	"strings"
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

type Day interface {
	Name() string
	Part1() string
	Part2() string
}

type WIP interface {
	WIP() string
}

type Visual interface {
	Visualize() tea.Model
}

func Solve(day Day) {
	t := time.Now()
	log.Printf("~~~~~~~~~~ %v ~~~~~~~~~~", day.Name())
	log.Printf("Part 1: %v\n", day.Part1())
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
	t = time.Now()
	log.Printf("Part 2: %v\n", day.Part2())
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
}

func SolveString(day Day) string {
	t := time.Now()
	s := fmt.Sprintf("~~~~~~~~~~ %v ~~~~~~~~~~\n", day.Name())
	s += fmt.Sprintf("Part 1: %v\n", day.Part1())
	s += fmt.Sprintf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
	t = time.Now()
	s += fmt.Sprintf("Part 2: %v\n", day.Part2())
	s += fmt.Sprintf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
	return s
}

func ReadInputAsString(path string) string {
	return ReadXAsString(path)
}

func ReadXAsString(name string) string {
	inputFile := name
	data, err := os.ReadFile(inputFile)
	if err != nil {
		log.Panicln("Failed to read file")
	}
	return strings.Trim(string(data), " \n")
}

// ReadInputAsLines does not include the new line characters
func ReadInputAsLines(path string) []string {
	return ReadXAsLines(path)
}

// ReadXAsLines does not include the new line characters
func ReadXAsLines(name string) []string {
	data := ReadXAsString(name)
	return strings.Split(data, "\n")
}

// ReadInputAsByteArray does not include the new line characters
func ReadInputAsByteArray(path string) [][]byte {
	return ReadXAsByteArray(path)
}

func ReadRawAsByteArray(input string) [][]byte {
	lines := strings.Split(input, "\n")
	bytes := make([][]byte, len(lines))
	for i := range bytes {
		bytes[i] = make([]byte, len(lines[0]))
	}
	for i, line := range lines {
		bytes[i] = []byte(line)
	}
	return bytes
}

// ReadXAsByteArray does not include the new line characters
func ReadXAsByteArray(name string) [][]byte {
	data := ReadXAsString(name)
	return ReadRawAsByteArray(data)
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

// Coordinates contain X (second layer of matrix), Y (first layer of matrix), Z (third layer of matrix)
type Coordinates struct {
	X int
	Y int
	Z int
}

type Direction int

// TODO: Make more complicated for 3d if needed
const (
	//NW Northwest X-1, Y-1
	NW Direction = iota
	//N North Y-1
	N
	//NE Northeast X+1, Y-1
	NE
	//W West X-1
	W
	//E East X+1
	E
	//SW Southwest Y-1, X-1
	SW
	//S South Y-1
	S
	//SE Southeast X+1, Y-1
	SE
)

const OFFGRID = "off grid"

func GetInDirection[K any](grid [][]K, location Coordinates, direction Direction) (K, Coordinates, error) {
	switch direction {
	case NW:
		if location.Y-1 >= 0 && location.X-1 >= 0 {
			return grid[location.Y-1][location.X-1], Coordinates{X: location.X - 1, Y: location.Y - 1}, nil
		}
	case N:
		if location.Y-1 >= 0 {
			return grid[location.Y-1][location.X], Coordinates{X: location.X, Y: location.Y - 1}, nil
		}
	case NE:
		if location.Y-1 >= 0 && location.X+1 < len(grid[0]) {
			return grid[location.Y-1][location.X+1], Coordinates{X: location.X + 1, Y: location.Y - 1}, nil
		}
	case W:
		if location.X-1 >= 0 {
			return grid[location.Y][location.X-1], Coordinates{X: location.X - 1, Y: location.Y}, nil
		}
	case E:
		if location.X+1 < len(grid[0]) {
			return grid[location.Y][location.X+1], Coordinates{X: location.X + 1, Y: location.Y}, nil
		}
	case SW:
		if location.Y+1 < len(grid) && location.X-1 >= 0 {
			return grid[location.Y+1][location.X-1], Coordinates{X: location.X - 1, Y: location.Y + 1}, nil
		}
	case S:
		if location.Y+1 < len(grid) {
			return grid[location.Y+1][location.X], Coordinates{X: location.X, Y: location.Y + 1}, nil
		}
	case SE:
		if location.Y+1 < len(grid) && location.X+1 < len(grid[0]) {
			return grid[location.Y+1][location.X+1], Coordinates{X: location.X + 1, Y: location.Y + 1}, nil
		}
	}

	return *new(K), *new(Coordinates), errors.New(OFFGRID)
}

func GetInDirectionIncludingOffGrid[K any](grid [][]K, location Coordinates, direction Direction) (K, Coordinates) {
	switch direction {
	case NW:
		if location.Y-1 >= 0 && location.X-1 >= 0 {
			return grid[location.Y-1][location.X-1], Coordinates{X: location.X - 1, Y: location.Y - 1}
		} else {
			return *new(K), Coordinates{X: location.X - 1, Y: location.Y - 1}
		}
	case N:
		if location.Y-1 >= 0 {
			return grid[location.Y-1][location.X], Coordinates{X: location.X, Y: location.Y - 1}
		} else {
			return *new(K), Coordinates{X: location.X - 1, Y: location.Y - 1}
		}
	case NE:
		if location.Y-1 >= 0 && location.X+1 < len(grid[0]) {
			return grid[location.Y-1][location.X+1], Coordinates{X: location.X + 1, Y: location.Y - 1}
		} else {
			return *new(K), Coordinates{X: location.X - 1, Y: location.Y - 1}
		}
	case W:
		if location.X-1 >= 0 {
			return grid[location.Y][location.X-1], Coordinates{X: location.X - 1, Y: location.Y}
		} else {
			return *new(K), Coordinates{X: location.X - 1, Y: location.Y - 1}
		}
	case E:
		if location.X+1 < len(grid[0]) {
			return grid[location.Y][location.X+1], Coordinates{X: location.X + 1, Y: location.Y}
		} else {
			return *new(K), Coordinates{X: location.X - 1, Y: location.Y - 1}
		}
	case SW:
		if location.Y+1 < len(grid) && location.X-1 >= 0 {
			return grid[location.Y+1][location.X-1], Coordinates{X: location.X - 1, Y: location.Y + 1}
		} else {
			return *new(K), Coordinates{X: location.X - 1, Y: location.Y - 1}
		}
	case S:
		if location.Y+1 < len(grid) {
			return grid[location.Y+1][location.X], Coordinates{X: location.X, Y: location.Y + 1}
		} else {
			return *new(K), Coordinates{X: location.X - 1, Y: location.Y - 1}
		}
	case SE:
		if location.Y+1 < len(grid) && location.X+1 < len(grid[0]) {
			return grid[location.Y+1][location.X+1], Coordinates{X: location.X + 1, Y: location.Y + 1}
		} else {
			return *new(K), Coordinates{X: location.X - 1, Y: location.Y - 1}
		}
	}

	panic("Shouldn't get here right now")
	return *new(K), *new(Coordinates)
}

// GetXInDirection returns current element + X-1 in direction
func GetXInDirection[K any](x int, grid [][]K, location Coordinates, direction Direction) ([]K, Coordinates, error) {
	ks := []K{grid[location.Y][location.X]}
	for x > 1 {
		val, loc, err := GetInDirection(grid, location, direction)
		if err != nil {
			return nil, *new(Coordinates), errors.New(OFFGRID)
		}
		location = loc
		ks = append(ks, val)
		x--
	}
	return ks, location, nil
}

type Neighbor[K any] struct {
	Val    K
	Coords Coordinates
	Dir    Direction
}

// GetNeighbors Ignores off grid errors and adds them anyway
func GetNeighbors[K any](grid [][]K, location Coordinates) []Neighbor[K] {
	var neighbors []Neighbor[K]
	up, c := GetInDirectionIncludingOffGrid(grid, location, N)
	neighbors = append(neighbors, Neighbor[K]{Val: up, Coords: c, Dir: N})

	left, c := GetInDirectionIncludingOffGrid(grid, location, W)
	neighbors = append(neighbors, Neighbor[K]{Val: left, Coords: c, Dir: W})

	right, c := GetInDirectionIncludingOffGrid(grid, location, E)
	neighbors = append(neighbors, Neighbor[K]{Val: right, Coords: c, Dir: E})

	down, c := GetInDirectionIncludingOffGrid(grid, location, S)
	neighbors = append(neighbors, Neighbor[K]{Val: down, Coords: c, Dir: S})

	return neighbors
}

// GetByteGrid only works with uniform length lines
func GetByteGrid(lines []string) [][]byte {
	var grid [][]byte
	for i := 0; i < len(lines); i++ {
		grid = append(grid, []byte(lines[i]))
	}
	return grid
}

func TurnRight(dir Direction) Direction {
	switch dir {
	case N:
		return E
	case E:
		return S
	case S:
		return W
	case W:
		return N
		// Not handling 45 degree turns :)
	default:
		return N
	}
}

func (loc1 Coordinates) Intersects(loc2 Coordinates) bool {
	if loc1.X == loc2.X || loc1.Y == loc2.Y {
		return true
	}
	return false
}

// PointInGrid checks if the given point fits within the bounds of the given grid. Only works for uniform grids
func PointInGrid[T any](p im.Point, g [][]T) bool {
	if p.X >= 0 && p.Y >= 0 && p.Y < len(g) && p.X < len(g[0]) {
		return true
	}
	return false
}

var DirectionChange = map[Direction]im.Point{
	NW: {X: -1, Y: -1},
	N:  {X: 0, Y: -1},
	NE: {X: 1, Y: -1},
	W:  {X: -1, Y: 0},
	E:  {X: 1, Y: 0},
	SW: {X: -1, Y: 1},
	S:  {X: 0, Y: 1},
	SE: {X: 1, Y: 1},
}

// GetPointNeighbors gets the adjacent neighbors with no diags
func GetPointNeighbors[K any](grid [][]K, point im.Point) []im.Point {
	var neighbors []im.Point

	nor := point.Add(DirectionChange[N])
	if PointInGrid(nor, grid) {
		neighbors = append(neighbors, nor)
	}
	eas := point.Add(DirectionChange[E])
	if PointInGrid(eas, grid) {
		neighbors = append(neighbors, eas)
	}
	sou := point.Add(DirectionChange[S])
	if PointInGrid(sou, grid) {
		neighbors = append(neighbors, sou)
	}
	wes := point.Add(DirectionChange[W])
	if PointInGrid(wes, grid) {
		neighbors = append(neighbors, wes)
	}

	return neighbors
}
