package main

import (
	day7 "github.com/rydwhelchel/advent-of-code/2015/Day07"
	day8 "github.com/rydwhelchel/advent-of-code/2015/Day08"
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
)

func main() {
	day7ctx := day7.Context{Path: "./Day07"}
	helpers.Solve(&day7ctx)
	day8ctx := day8.Context{Path: "./Day08"}
	helpers.Solve(&day8ctx)
}
