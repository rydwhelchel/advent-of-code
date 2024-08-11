package main

import (
	"fmt"
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
	b := time.Now()
	log.Printf("Part 1: %v\n", part1(data))
	log.Printf("	in %.6f seconds\n", time.Now().Sub(b).Seconds())
	c := time.Now()
	log.Printf("Part 2: %v\n", part2(data))
	log.Printf("	in %.6f seconds\n", time.Now().Sub(c).Seconds())
}

func part1(data []byte) int {
	houses := map[string]int{}
	var x, y int
	houses["0, 0"]++
	for _, b := range data {
		switch b {
		case '>':
			x++
		case '<':
			x--
		case 'v':
			y++
		case '^':
			y--
		}
		c := coord(x, y)
		houses[c]++
	}
	houseCount := 0
	for _, v := range houses {
		if v > 0 {
			houseCount++
		}
	}
	return houseCount
}

func coord(x, y int) string {
	return fmt.Sprintf("%v, %v", x, y)
}

func part2(data []byte) int {
	houses := map[string]int{}
	var x, y int
	houses["0, 0"]++
	for i, b := range data {
		if i%2 == 0 {
			continue
		}
		switch b {
		case '>':
			x++
		case '<':
			x--
		case 'v':
			y++
		case '^':
			y--
		}
		c := coord(x, y)
		houses[c]++
	}

	var xs, ys int
	for i, b := range data {
		if i%2 == 1 {
			continue
		}
		switch b {
		case '>':
			xs++
		case '<':
			xs--
		case 'v':
			ys++
		case '^':
			ys--
		}
		c := coord(xs, ys)
		houses[c]++
	}

	houseCount := 0
	for _, v := range houses {
		if v > 0 {
			houseCount++
		}
	}
	return houseCount
}
