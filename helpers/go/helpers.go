package helpers

import (
	"errors"
	"log"
	"os"
	"strings"
	"time"
)

type Day interface {
	Name() string
	Part1() string
	Part2() string
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

func ReadInputAsString(name string) string {
	return ReadXAsString(name + "/input")
}

func ReadXAsString(name string) string {
	inputFile := name
	data, err := os.ReadFile(inputFile)
	if err != nil {
		log.Panicln("Failed to read file")
	}
	return strings.Trim(string(data), "\n")
}

// ReadInputAsLines does not include the new line characters
func ReadInputAsLines(day string) []string {
	return ReadXAsLines(day + "/input")
}

// ReadXAsLines does not include the new line characters
func ReadXAsLines(name string) []string {
	data := ReadXAsString(name)
	return strings.Split(data, "\n")
}

// ReadInputAsByteArrays does not include the new line characters
func ReadInputAsByteArrays(day string) [][]byte {
	return ReadXAsByteArray(day + "/input")
}

// ReadXAsByteArray does not include the new line characters
func ReadXAsByteArray(name string) [][]byte {
	data := ReadXAsString(name)
	lines := strings.Split(data, "\n")
	bytes := make([][]byte, len(lines))
	for i := range bytes {
		bytes[i] = make([]byte, len(lines[0]))
	}
	for _, line := range lines {
		bytes = append(bytes, []byte(line))
	}
	return bytes
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

//func getNeighbors[K any](grid [][]K, location Coordinates)

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
