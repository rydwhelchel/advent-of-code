package main

import (
	"log"
	"strconv"

	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
)

func main() {
	log.Printf("Part1: %v\n", Part1())
	log.Printf("Part2: %v\n", Part2())
}

type direction int

const (
	l direction = iota
	r
)

type instruction struct {
	direction direction
	count     int
}

func parseLine(line string) (ins instruction) {
	if len(line) < 2 {
		log.Fatal("unable to parse line")
	}

	ins = instruction{}
	switch line[0] {
	case 'L':
		ins.direction = l
	case 'R':
		ins.direction = r
	default:
		log.Fatal("unknown direction")
	}
	i, err := strconv.Atoi(line[1:])
	if err != nil {
		log.Fatal("unable to parse count")
	}
	ins.count = i
	return
}

func Part1() int {
	lines := helpers.ReadInputAsLines("./input")

	ticker := 50
	pass := 0
	for _, line := range lines {
		ins := parseLine(line)

		switch ins.direction {
		case l:
			ticker -= ins.count
		case r:
			ticker += ins.count
		}

		ticker = (ticker + 100) % 100

		if ticker == 0 {
			pass += 1
		}
	}
	return pass
}

func Part2() int {
	lines := helpers.ReadInputAsLines("./input")

	ticker := 50
	pass := 0
	for _, line := range lines {
		ins := parseLine(line)

		crosses := 0

		// just brute force it 🍤
		for range ins.count {
			switch ins.direction {
			case l:
				ticker -= 1
			case r:
				ticker += 1
			}
			if ticker < 0 {
				ticker += 100
			} else if ticker > 99 {
				ticker -= 100
			}

			if ticker == 0 {
				crosses += 1
			}
		}

		pass += crosses
	}
	return pass
}
