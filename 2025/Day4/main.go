package main

import (
	"log"

	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
)

const (
	targetEl = '@'
)

func main() {
	log.Printf("Part1: %v\n", Part1())
	log.Printf("Part2: %v\n", Part2())
}

func Part1() (count int) {
	array := helpers.ReadInputAsByteArray("./input")

	for y, row := range array {
		for x, element := range row {
			if element == targetEl && countNeighbors(array, x, y) < 4 {
				count += 1
			}
		}
	}
	return
}

func countNeighbors(array [][]byte, x, y int) (count int) {
	coords := helpers.Coordinates{X: x, Y: y}
	nbs := helpers.GetNeighborsWithDiags(array, coords)
	for _, nb := range nbs {
		if nb.Val == targetEl {
			count += 1
		}
	}
	return
}

func Part2() (count int) {
	array := helpers.ReadInputAsByteArray("./input")

	removed := -1
	for removed != 0 {
		removed = 0
		for y, row := range array {
			for x, element := range row {
				if element == targetEl && countNeighbors(array, x, y) < 4 {
					array[y][x] = '.'
					removed += 1
				}
			}
		}
		count += removed
	}
	return
}
