package main

import (
	"github.com/rydwhelchel/advent-of-code/2024/Day01"
	"github.com/rydwhelchel/advent-of-code/2024/Day02"
	"github.com/rydwhelchel/advent-of-code/2024/Day03"
	"github.com/rydwhelchel/advent-of-code/2024/Day04"
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
)

func main() {
	day1Ctx := Day01.Context{Path: "./Day01"}
	helpers.Solve(&day1Ctx)
	day2Ctx := Day02.Context{Path: "./Day02"}
	helpers.Solve(&day2Ctx)
	day3Ctx := Day03.Context{Path: "./Day03"}
	helpers.Solve(&day3Ctx)
	day4Ctx := Day04.Context{Path: "./Day04"}
	helpers.Solve(&day4Ctx)
}
