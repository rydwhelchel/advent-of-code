package main

import (
	"log"
	"strconv"
	"strings"

	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
)

func main() {
	log.Printf("Part1: %v\n", Part1())
	log.Printf("Part2: %v\n", Part2())
}

func Part1() int {
	lines := helpers.ReadInputAsString("./input")
	ranges := strings.Split(lines, ",")

	sum := 0

	for _, r := range ranges {
		bounds := strings.Split(r, "-")
		if len(bounds) != 2 {
			log.Fatalf("incorrect length of bounds: %v\n", bounds)
		}
		lowerBound, _ := strconv.Atoi(bounds[0])
		upperBound, _ := strconv.Atoi(bounds[1])

		for i := lowerBound; i <= upperBound; i++ {
			num := strconv.Itoa(i)
			if mid := len(num) / 2; num[0:mid] == num[mid:] {
				sum += i
			}
		}
	}

	return sum
}

func Part2() int {
	lines := helpers.ReadInputAsString("./input")
	ranges := strings.Split(lines, ",")

	sum := 0

	for _, r := range ranges {
		bounds := strings.Split(r, "-")
		if len(bounds) != 2 {
			log.Fatalf("incorrect length of bounds: %v\n", bounds)
		}
		lowerBound, _ := strconv.Atoi(bounds[0])
		upperBound, _ := strconv.Atoi(bounds[1])

		for i := lowerBound; i <= upperBound; i++ {
			num := strconv.Itoa(i)
			rs := ""
			// BRUTEFORCE 💪
			for _, c := range num {
				rs += string(c)
				occurrences := strings.Count(num, rs)
				if len(rs)*occurrences == len(num) && occurrences >= 2 {
					sum += i
					break
				}
			}
		}
	}

	return sum
}
