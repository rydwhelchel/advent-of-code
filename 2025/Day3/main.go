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

func Part1() (sum int) {
	lines := helpers.ReadInputAsLines("./input")

	for _, line := range lines {
		maxFirstDigit, idx := getMax(line[:len(line)-1])
		maxSecondDigit, _ := getMax(line[idx+1:])
		n, err := strconv.Atoi(strconv.Itoa(maxFirstDigit) + strconv.Itoa(maxSecondDigit))
		if err != nil {
			log.Fatalf("error atoi-ing: %v", err)
		}
		sum += n
	}

	return
}

func getMax(line string) (val, index int) {
	val, index = 0, 0
	for i, b := range line {
		num := int(b - '0')
		if num > val {
			val = num
			index = i
		}
	}
	return val, index
}

func Part2() (sum int) {
	lines := helpers.ReadInputAsLines("./input")

	for _, line := range lines {
		maxVal := ""
		idx := 0
		for len(maxVal) < 12 {
			dig, i := getMax(line[idx : len(line)-(11-len(maxVal))])
			idx += i + 1
			maxVal += strconv.Itoa(dig)
		}

		s, e := strconv.Atoi(maxVal)
		if e != nil {
			log.Fatalf("error atoi-ing: %v", e)
		}
		sum += s
	}

	return
}
