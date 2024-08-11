package main

import (
	"log"
	"os"
)

func main() {
	input, err := os.ReadFile("./input.txt")
	if err != nil {
		log.Panicln("Failed to read file \"./input1.txt\"")
	}

	log.Printf("Part 1: %v\n", part1(input))
	log.Printf("Part 2: %v\n", part2(input))
}

func part1(input []byte) int {
	sum := 0
	for _, v := range input {
		switch v {
		case byte('('):
			sum++
		case byte(')'):
			sum--
		default:
			log.Panicln("Wrong bytes homie")
		}
	}
	return sum
}

func part2(input []byte) int {
	sum := 0
	for i, v := range input {
		switch v {
		case byte('('):
			sum++
		case byte(')'):
			sum--
		default:
			log.Panicln("Wrong bytes homie")
		}
		if sum < 0 {
			return i + 1
		}
	}
	return sum
}
