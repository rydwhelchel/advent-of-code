package main

import (
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	inputName := "./input.txt"
	data, err := os.ReadFile(inputName)
	if err != nil {
		log.Panicln("Coudln't read input")
	}
	log.Printf("Part 1: %v\n", part1(string(data)))
	log.Printf("Part 2: %v\n", part2(string(data)))
}

func part1(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		sum += calcWrappingPaper(getLengthWidthHeight(line))
	}
	return sum
}

func getLengthWidthHeight(line string) (l, w, h int) {
	dimensions := strings.Split(line, "x")
	var le, wi, he int
	if len(dimensions) == 3 {
		le, _ = strconv.Atoi(dimensions[0])
		wi, _ = strconv.Atoi(dimensions[1])
		he, _ = strconv.Atoi(dimensions[2])
	}
	return le, wi, he
}

func calcWrappingPaper(l, w, h int) int {
	smallestSide := min(l*w, w*h, l*h)
	return 2*l*w + 2*w*h + 2*h*l + smallestSide
}

func part2(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		sum += calcRibbonLength(getLengthWidthHeight(line))
	}
	return sum
}

func calcRibbonLength(l, w, h int) int {
	smallestPerimeter := min(l*2+w*2, l*2+h*2, w*2+h*2)
	cubicVolume := l * w * h
	return smallestPerimeter + cubicVolume
}
