package main

import (
	"fmt"
	"os"
	"strconv"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/rydwhelchel/advent-of-code/2024/Day01"
	"github.com/rydwhelchel/advent-of-code/2024/Day02"
	"github.com/rydwhelchel/advent-of-code/2024/Day03"
	"github.com/rydwhelchel/advent-of-code/2024/Day04"
	"github.com/rydwhelchel/advent-of-code/2024/Day05"
	"github.com/rydwhelchel/advent-of-code/2024/Day06"
	"github.com/rydwhelchel/advent-of-code/2024/Day07"
	"github.com/rydwhelchel/advent-of-code/2024/Day08"
	"github.com/rydwhelchel/advent-of-code/2024/Day09"
	"github.com/rydwhelchel/advent-of-code/2024/Day10"
	"github.com/rydwhelchel/advent-of-code/2024/Day11"
	"github.com/rydwhelchel/advent-of-code/2024/Day12"
	"github.com/rydwhelchel/advent-of-code/2024/Day13"
	"github.com/rydwhelchel/advent-of-code/2024/Day14"
	"github.com/rydwhelchel/advent-of-code/2024/Day17"
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
)

func main() {
	day1Ctx := Day01.Context{Path: "./Day01/input"}
	day2Ctx := Day02.Context{Path: "./Day02/input"}
	day3Ctx := Day03.Context{Path: "./Day03/input"}
	day4Ctx := Day04.Context{Path: "./Day04/input"}
	day5Ctx := Day05.Context{Path: "./Day05/input"}
	day6Ctx := Day06.Context{Path: "./Day06/input"}
	day7Ctx := Day07.Context{Path: "./Day07/input"}
	day8Ctx := Day08.Context{Path: "./Day08/input"}
	day9Ctx := Day09.Context{Path: "./Day09/input"}
	day10Ctx := Day10.Context{Path: "./Day10/input"}
	day11Ctx := Day11.Context{Path: "./Day11/input"}
	day12Ctx := Day12.Context{Path: "./Day12/input"}
	day13Ctx := Day13.Context{Path: "./Day13/input"}
	day14Ctx := Day14.Context{Path: "./Day14/input"}
	day17Ctx := Day17.Context{Path: "./Day17/input"}

	args := os.Args[1:]
	if len(args) == 1 {
		val, err := strconv.Atoi(args[0])
		if err != nil {
			panic("invalid args")
		}
		switch val {
		case 1:
			helpers.Solve(&day1Ctx)
		case 2:
			helpers.Solve(&day2Ctx)
		case 3:
			helpers.Solve(&day3Ctx)
		case 4:
			helpers.Solve(&day4Ctx)
		case 5:
			helpers.Solve(&day5Ctx)
		case 6:
			helpers.Solve(&day6Ctx)
		case 7:
			helpers.Solve(&day7Ctx)
		case 8:
			helpers.Solve(&day8Ctx)
		case 9:
			helpers.Solve(&day9Ctx)
		case 10:
			helpers.Solve(&day10Ctx)
		case 11:
			helpers.Solve(&day11Ctx)
		case 12:
			helpers.Solve(&day12Ctx)
		case 13:
			helpers.Solve(&day13Ctx)
		case 14:
			helpers.Solve(&day14Ctx)
		case 17:
			helpers.Solve(&day17Ctx)
		}
		return
	} else if len(args) > 1 {
		panic("too many args?")
	}

	// If no args are passed, go to cmdline selector
	ctxs := []helpers.Day{
		&day1Ctx,
		&day2Ctx,
		&day3Ctx,
		&day4Ctx,
		&day5Ctx,
		&day6Ctx,
		&day7Ctx,
		&day8Ctx,
		&day9Ctx,
		&day10Ctx,
		&day11Ctx,
		&day12Ctx,
		&day13Ctx,
		&day14Ctx,
		&day17Ctx,
	}

	model := UIModel{
		choices: ctxs,
		cursor:  0,
	}

	p := tea.NewProgram(model, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}

type UIModel struct {
	choices  []helpers.Day
	cursor   int
	selected bool
}

func (m UIModel) Init() tea.Cmd {
	return nil
}

func (m UIModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit
		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down", "j":
			if m.cursor < len(m.choices)-1 {
				m.cursor++
			}
		case "enter", " ":
			m.selected = true
			// TODO: Hmm almost, not quite working -- check here: https://shi.foo/weblog/multi-view-interfaces-in-bubble-tea
			// var i interface{} = m.choices[m.cursor]
			// v, isVisual := i.(helpers.Visual)
			// if isVisual {
			// 	return v.Visualize().Update(msg)
			// }
			// TODO: Remove clear screen call, this is a little overkill
			return m, tea.ClearScreen
		}
	}
	return m, nil
}

func (m UIModel) View() string {
	if m.selected {
		chose := m.choices[m.cursor]
		s := "Solving " + chose.Name() + "\n\n"
		// TODO: Make this able to solve part1 separately from part2 (so if part2 is long, we still see part1 output quickly)
		s += helpers.SolveString(chose)
		return s
	}
	s := "Choose which day to solve:\n\n"
	for i, choice := range m.choices {
		cursor := " " // no cursor
		if m.cursor == i {
			cursor = ">" // cursor!
		}

		// Check if the given Day also is a WIP
		var i interface{} = choice
		v, isAWIP := i.(helpers.WIP)

		if isAWIP {
			s += fmt.Sprintf("%s %s %s\n", cursor, choice.Name(), v.WIP())
		} else {
			s += fmt.Sprintf("%s %s\n", cursor, choice.Name())
		}
	}

	// The footer
	s += "\nPress q to quit.\n"

	// Send the UI for rendering
	return s
}
