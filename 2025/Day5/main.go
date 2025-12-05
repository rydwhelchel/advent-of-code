package main

import (
	"cmp"
	"log"
	"slices"
	"strconv"
	"strings"

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
	lines := helpers.ReadInputAsLines("./input")

	ranges, lines := parseIngredientRanges(lines)
	for _, line := range lines {
		for _, r := range ranges {
			id, err := strconv.Atoi(line)
			if err != nil {
				log.Fatalf("failed to parse id: %v", err)
			}
			if r.contains(id) {
				count += 1
				break
			}
		}
	}
	return
}

func Part2() (count int) {
	lines := helpers.ReadInputAsLines("./input")

	ranges, _ := parseIngredientRanges(lines)
	slices.SortFunc(ranges, func(a ingredientRange, b ingredientRange) int {
		return cmp.Compare(a.low, b.low)
	})
	mergeIdx := 0
	mergedRanges := []ingredientRange{ranges[0]}
	for _, r := range ranges[1:] {
		if newRange, overlapping := r.merge(mergedRanges[mergeIdx]); overlapping {
			mergedRanges[mergeIdx] = newRange
		} else {
			mergedRanges = append(mergedRanges, r)
			mergeIdx += 1
		}
	}

	for _, r := range mergedRanges {
		count += (r.high - r.low) + 1 // inclusive ranges
	}

	return
}

// returns the remaining lines
func parseIngredientRanges(lines []string) ([]ingredientRange, []string) {
	ranges := []ingredientRange{}
	for i := range lines {
		// if we've reached the terminus of ranges
		if lines[i] == "" {
			return ranges, lines[i+1:]
		}
		parts := strings.Split(lines[i], "-")
		lowerBound, err := strconv.Atoi(parts[0])
		if err != nil {
			log.Fatalf("failed to parse lowerbound: %v", parts)
		}
		upperBound, err := strconv.Atoi(parts[1])
		if err != nil {
			log.Fatalf("failed to parse upperbound: %v", parts)
		}
		ranges = append(ranges, ingredientRange{low: lowerBound, high: upperBound})
	}
	log.Fatal("unexpected EOF")
	return nil, nil // unreachable
}

type ingredientRange struct {
	low, high int
}

func (i ingredientRange) contains(n int) bool {
	if n >= i.low && n <= i.high {
		return true
	}
	return false
}

func (a ingredientRange) merge(b ingredientRange) (ingredientRange, bool) {
	newRange := ingredientRange{}
	if a.high < b.low || b.high < a.low {
		return newRange, false
	}
	newRange.low = min(a.low, b.low)
	newRange.high = max(a.high, b.high)
	return newRange, true
}
