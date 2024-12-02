package Day01

import (
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
	"log"
	"slices"
	"strconv"
	"strings"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 1" }

func (ctx *Context) Part1() string {
	lines := helpers.ReadInputAsLines(ctx.Path)

	var right []int
	var left []int
	for _, line := range lines {
		if line == "" {
			break
		}
		splitLine := strings.Split(line, "   ")
		if len(splitLine) != 2 {
			log.Panicf("Split line not long enough %v, len: %v", splitLine, len(splitLine))
		}
		for _, x := range splitLine {
			x = strings.Trim(x, " ")
		}
		lint, err := strconv.Atoi(splitLine[0])
		if err != nil {
			log.Panicf("Unable to format to int %v", splitLine[0])
		}
		rint, err := strconv.Atoi(splitLine[1])
		if err != nil {
			log.Panicf("Unable to format to int %v", splitLine[1])
		}
		left = append(left, lint)
		right = append(right, rint)
	}
	slices.Sort(left)
	slices.Sort(right)
	sum := 0
	for i := range left {
		sum += helpers.Abs(left[i] - right[i])
	}
	return strconv.Itoa(sum)
}
func (ctx *Context) Part2() string {
	lines := helpers.ReadInputAsLines(ctx.Path)
	rightMap := make(map[int]int)
	var left []int
	for _, line := range lines {
		if line == "" {
			break
		}
		splitLine := strings.Split(line, "   ")
		if len(splitLine) != 2 {
			log.Panicf("Split line not long enough %v, len: %v", splitLine, len(splitLine))
		}
		for _, x := range splitLine {
			x = strings.Trim(x, " ")
		}
		lint, err := strconv.Atoi(splitLine[0])
		if err != nil {
			log.Panicf("Unable to format to int %v", splitLine[0])
		}
		rint, err := strconv.Atoi(splitLine[1])
		if err != nil {
			log.Panicf("Unable to format to int %v", splitLine[1])
		}
		left = append(left, lint)
		rightMap[rint]++
	}

	sum := 0
	for _, v := range left {
		sum += v * rightMap[v]
	}
	return strconv.Itoa(sum)
}
