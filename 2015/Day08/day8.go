package day8

import (
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
	"strconv"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 8" }

func (ctx *Context) Part1() string {
	lines := helpers.ReadInputAsLines(ctx.Path)

	sum := 0
	for _, line := range lines {
		charCount := len(line)
		actualCount := 0
		for i := 0; i < len(line); i++ {
			if line[i] == '\\' {
				if line[i+1] == 'x' {
					i += 3
				} else {
					i += 1
				}
			} else if line[i] == '"' {
				continue
			}
			actualCount += 1
		}
		sum += charCount - actualCount
	}
	return strconv.Itoa(sum)
}
func (ctx *Context) Part2() string {
	lines := helpers.ReadInputAsLines(ctx.Path)
	sum := 0
	for _, line := range lines {
		charCount := len(line)
		actualCount := charCount
		for i := 0; i < len(line); i++ {
			if line[i] == '"' {
				actualCount += 1
			} else if line[i] == '\\' {
				actualCount += 1
			}
		}
		sum += actualCount - charCount + 2
	}
	return strconv.Itoa(sum)
}
