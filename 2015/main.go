package main

import (
	"log"
	"os"
	"time"
)

func main() {
	inputFile := "./input.txt"
	data, err := os.ReadFile(inputFile)
	if err != nil {
		log.Panicln("Failed to read file")
	}
	t := time.Now()
	log.Printf("Part 1: %v\n", part1(data))
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
	t = time.Now()
	log.Printf("Part 2: %v\n", part2(data))
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
}

func part1(data []byte) int {
	return 0
}

func part2(data []byte) int {
	return 0
}
