package Day11

import (
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
	"strconv"
	"strings"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 11" }

func (ctx *Context) Part1() string {
	input := helpers.ReadInputAsString(ctx.Path)

	vals := strings.Split(input, " ")

	current := make(map[string]int)

	for _, val := range vals {
		current[val]++
	}

	// blinks!
	for range 25 {
		next := make(map[string]int)
		for k, v := range current {
			o1, o2 := calcResult(k)
			next[o1] += v
			if o2 != "" {
				next[o2] += v
			}
		}
		current = next
	}
	sum := 0
	for _, v := range current {
		sum += v
	}
	return strconv.Itoa(sum)
}

// calcResult returns the result of one blink on the given stone, if only one stone is output,the second return is empty
func calcResult(stone string) (string, string) {
	if stone == "0" {
		return "1", ""
	} else if len(stone)%2 == 0 {
		val1, err := strconv.Atoi(stone[:len(stone)/2])
		if err != nil {
			panic("pancakes")
		}
		val2, err := strconv.Atoi(stone[len(stone)/2:])
		if err != nil {
			panic("val2")
		}
		return strconv.Itoa(val1), strconv.Itoa(val2)
	} else {
		val, err := strconv.Atoi(stone)
		if err != nil {
			panic("oops")
		}
		return strconv.Itoa(val * 2024), ""
	}
}

func (ctx *Context) Part2() string {
	input := helpers.ReadInputAsString(ctx.Path)

	vals := strings.Split(input, " ")

	current := make(map[string]int)

	for _, val := range vals {
		current[val]++
	}

	// a lotta blinks!
	for range 75 {
		next := make(map[string]int)
		for k, v := range current {
			o1, o2 := calcResult(k)
			next[o1] += v
			if o2 != "" {
				next[o2] += v
			}
		}
		current = next
	}
	sum := 0
	for _, v := range current {
		sum += v
	}
	return strconv.Itoa(sum)
}
