package Day03

import (
	"encoding/json"
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
	"regexp"
	"strconv"
	"strings"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 3" }

func (ctx *Context) Part1() string {
	input := helpers.ReadInputAsString(ctx.Path)

	return strconv.Itoa(countMultiples(input))
}

func (ctx *Context) Part2() string {
	input := helpers.ReadInputAsString(ctx.Path)
	parsed := ""
	on := true
	i := 0
	j := 0
	for {
		if on {
			i = strings.Index(input[j:], "don't()")
			if i == -1 {
				break
			}
			parsed += input[j : j+i]
			j = j + i
			on = false
		} else {
			i = strings.Index(input[j:], "do()")
			if i == -1 {
				break
			}
			j = j + i
			on = true
		}
	}
	sum := countMultiples(parsed)
	if on {
		sum += countMultiples(input[j:])
	}
	return strconv.Itoa(sum)
}

func countMultiples(input string) int {
	// returns a capture group of num,num
	r := regexp.MustCompile(`mul\((\d{1,3},\d{1,3})\)`)
	pairs := r.FindAllStringSubmatch(input, -1)
	sum := 0
	for _, pair := range pairs {
		var nums []int
		err := json.Unmarshal([]byte("["+pair[1]+"]"), &nums)
		if err != nil {
			panic("oopsie unmarshal error")
		}
		if len(nums) != 2 {
			panic("oopsie wrong length")
		}
		sum += nums[0] * nums[1]
	}
	return sum
}
