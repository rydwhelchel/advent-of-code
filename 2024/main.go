package main

import (
	"github.com/rydwhelchel/advent-of-code/2024/Day01"
	"github.com/rydwhelchel/advent-of-code/2024/Day02"
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
)

func main() {
	day1Ctx := Day01.Context{Path: "./Day01"}
	helpers.Solve(&day1Ctx)
	day2Ctx := Day02.Context{Path: "./Day02"}
	helpers.Solve(&day2Ctx)
}
