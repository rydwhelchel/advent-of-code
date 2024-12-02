package Day02

import (
	"encoding/json"
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
	"slices"
	"strconv"
	"strings"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 2" }

func (ctx *Context) Part1() string {
	lines := helpers.ReadInputAsLines(ctx.Path)

	var count int
	for _, line := range lines {
		var levels []int
		err := json.Unmarshal([]byte("["+strings.ReplaceAll(line, " ", ",")+"]"), &levels)
		if err != nil {
			panic(err)
		}

		if checkLevels(levels) {
			count++
		}
	}

	return strconv.Itoa(count)
}

func (ctx *Context) Part2() string {
	lines := helpers.ReadInputAsLines(ctx.Path)

	var count int
	for _, line := range lines {
		var levels []int
		err := json.Unmarshal([]byte("["+strings.ReplaceAll(line, " ", ",")+"]"), &levels)
		if err != nil {
			panic(err)
		}

		if checkLevels(levels) {
			count++
		} else {
			// brute force, if it isn't valid, just try deleting one element at a time and checking it
			for j := range levels {
				sliced := slices.Delete(slices.Clone(levels), j, j+1)
				if checkLevels(sliced) {
					count++
					break
				}
			}
		}
	}

	return strconv.Itoa(count)
}

func checkLevels(levels []int) bool {
	// technically levels with len 1 could be valid, but that doesn't exist in data :)
	if len(levels) < 2 {
		return false
	}
	par := levels[1] - levels[0]

	for j := 0; j < len(levels)-1; j++ {
		prev := levels[j]
		next := levels[j+1]
		diff := next - prev
		// If parity is negative (desc) && diff is positive, will be less than 0 &viceversa
		if diff*par <= 0 || diff < -3 || diff > 3 {
			return false
		}
	}
	return true
}
