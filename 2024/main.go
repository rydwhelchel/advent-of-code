package main

import (
	"github.com/rydwhelchel/advent-of-code/2024/Day01"
	"github.com/rydwhelchel/advent-of-code/2024/Day02"
	"github.com/rydwhelchel/advent-of-code/2024/Day03"
	"github.com/rydwhelchel/advent-of-code/2024/Day04"
	"github.com/rydwhelchel/advent-of-code/2024/Day05"
	"github.com/rydwhelchel/advent-of-code/2024/Day06"
	"github.com/rydwhelchel/advent-of-code/2024/Day07"
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
	day5Ctx := Day05.Context{Path: "./Day05"}
	helpers.Solve(&day5Ctx)
	day6Ctx := Day06.Context{Path: "./Day06"}
	helpers.Solve(&day6Ctx)
	day7Ctx := Day07.Context{Path: "./Day07"}
	helpers.Solve(&day7Ctx)
}
