package main

import (
	"log"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

func main() {
	inputFile := "./input.txt"
	data, err := os.ReadFile(inputFile)
	if err != nil {
		log.Panicln("Failed to read file")
	}
	str := string(data)
	lines := strings.Split(str, "\n")

	commands := []command{}
	for _, v := range lines {
		if len(v) == 0 {
			break
		}
		commands = append(commands, parseCommand(v))
	}

	t := time.Now()
	log.Printf("Part 1: %v\n", part1(commands))
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
	t = time.Now()
	log.Printf("Part 2: %v\n", part2(commands))
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())

}

type coord struct {
	x int
	y int
}

type command struct {
	com    string
	coord1 coord
	coord2 coord
}

func parseCommand(line string) command {
	// find index of first num, split
	var splitIndex int
	for i, c := range line {
		// if the rune is a digit
		if c >= '0' && c <= '9' {
			splitIndex = i
			break
		}
	}
	com := strings.TrimSpace(line[0:splitIndex])
	rem := strings.Split(line[splitIndex:], " through ")

	rem1 := strings.Split(rem[0], ",")
	rem2 := strings.Split(rem[1], ",")

	x1, err := strconv.Atoi(rem1[0])
	if err != nil {
		log.Fatalf("critical error converting str %v", rem1[0])
	}
	y1, err := strconv.Atoi(rem1[1])
	if err != nil {
		log.Fatalf("critical error converting str %v\n", rem1[1])
	}
	x2, err := strconv.Atoi(rem2[0])
	if err != nil {
		log.Fatalf("critical error converting str %v\n", rem2[0])
	}
	y2, err := strconv.Atoi(rem2[1])
	if err != nil {
		log.Fatalf("critical error converting str %v\n", rem2[1])
	}

	coord1 := coord{x: x1, y: y1}
	coord2 := coord{x: x2, y: y2}

	return command{com: com, coord1: coord1, coord2: coord2}
}

// this is a brute force approach, I think it may be more memory
// efficient to use a map so that we're not tracking indexes we aren't touching
// may not be worth going to the heap though
func part1(commands []command) int {
	lights := make([][]bool, 1000)
	for i := range lights {
		lights[i] = make([]bool, 1000)
	}

	for _, command := range commands {
		switch command.com {
		case "turn on":
			turn(&lights, true, command)
		case "turn off":
			turn(&lights, false, command)
		case "toggle":
			toggle(&lights, command)
		default:
			log.Fatalf("This ain't a command! %v\n", command.com)
		}
	}

	count := 0
	for _, v := range lights {
		for _, on := range v {
			if on {
				count++
			}
		}
	}

	return count
}

func turn(lights *[][]bool, arg bool, command command) {
	x1 := command.coord1.x
	x2 := command.coord2.x
	y1 := command.coord1.y
	y2 := command.coord2.y
	for x := x1; x <= x2; x++ {
		for y := y1; y <= y2; y++ {
			(*lights)[x][y] = arg
		}
	}
}

func toggle(lights *[][]bool, command command) {
	x1 := command.coord1.x
	x2 := command.coord2.x
	y1 := command.coord1.y
	y2 := command.coord2.y
	for x := x1; x <= x2; x++ {
		for y := y1; y <= y2; y++ {
			(*lights)[x][y] = !(*lights)[x][y]
		}
	}
}

func part2(commands []command) int {
	lights := make([][]int, 1000)
	for i := range lights {
		lights[i] = make([]int, 1000)
	}

	for _, command := range commands {
		switch command.com {
		case "turn on":
			turnDial(&lights, 1, command)
		case "turn off":
			turnDial(&lights, -1, command)
		case "toggle":
			turnDial(&lights, 2, command)
		default:
			log.Fatalf("This ain't a command! %v\n", command.com)
		}
	}

	count := 0
	for _, v := range lights {
		for _, brightness := range v {
			count += brightness
		}
	}

	return count
}

func turnDial(lights *[][]int, arg int, command command) {
	x1 := command.coord1.x
	x2 := command.coord2.x
	y1 := command.coord1.y
	y2 := command.coord2.y
	for x := x1; x <= x2; x++ {
		for y := y1; y <= y2; y++ {
			(*lights)[x][y] = int(math.Max(float64((*lights)[x][y]+arg), 0))
		}
	}
}
