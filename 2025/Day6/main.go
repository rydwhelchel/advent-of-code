package main

import (
	"log"
	"strconv"
	"strings"
	"unicode"

	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
)

const (
	targetEl = '@'
)

func main() {
	log.Printf("Part1: %v\n", Part1())
	log.Printf("Part2: %v\n", Part2())
}

const (
	mul = iota
	add
)

type problem struct {
	nums     []int
	operator int
}

func Part1() (count int) {
	lines := helpers.ReadInputAsLines("./input")

	problems := parseProblems(lines)
	count = solveProblems(problems)

	return
}

func solveProblems(problems []*problem) (count int) {
	for _, p := range problems {
		acc := 0
		switch p.operator {
		case mul:
			acc = 1
			for _, n := range p.nums {
				acc *= n
			}
		case add:
			for _, n := range p.nums {
				acc += n
			}
		}
		count += acc
	}
	return
}

func parseProblems(lines []string) []*problem {
	// last line should be the line with operators
	elements := make([][]string, 0)
	for _, line := range lines {
		elements = append(elements, strings.Fields(line))
	}

	lenRow := len(elements[0])
	for _, element := range elements[1:] {
		if len(element) != lenRow {
			// doublechecking my assumptions
			log.Fatalf("rows are not of equal length")
		}
	}

	var problems []*problem
	for i := range lenRow {
		p := &problem{nums: make([]int, 0)}
		for j := range len(elements) - 1 {
			num, err := strconv.Atoi(elements[j][i])
			if err != nil {
				log.Fatalf("failed to parse int: %v", err)
			}
			p.nums = append(p.nums, num)
		}
		if op := elements[len(elements)-1][i]; op == "*" {
			p.operator = mul
		} else if op == "+" {
			p.operator = add
		} else {
			log.Fatalf("failed to parse op, unexpected op: %v", op)
		}

		problems = append(problems, p)
	}

	return problems
}

func Part2() (count int) {
	lines := helpers.ReadInputAsLines("./input")

	problems := parseEvilCephalopodProblems(lines)
	count = solveProblems(problems)

	return
}

// 🐙
func parseEvilCephalopodProblems(lines []string) (problems []*problem) {
	// all lines are equal length, even if they end in whitespace
	length := len(lines[0])
	currProblem := &problem{}
	for i := range length {
		numStr := ""
		for _, line := range lines[:len(lines)-1] { // don't parse op line
			if c := line[i]; unicode.IsDigit(rune(c)) {
				numStr += string(c)
			}
		}
		if numStr == "" {
			// this problem ended
			problems = append(problems, currProblem)
			currProblem = &problem{}
			continue
		}
		num, err := strconv.Atoi(numStr)
		if err != nil {
			log.Fatalf("Failed to parse num: %v", err)
		}
		currProblem.nums = append(currProblem.nums, num)
	}
	problems = append(problems, currProblem)

	// now parse operators
	opLine := strings.Fields(lines[len(lines)-1])
	if len(problems) != len(opLine) {
		log.Fatalf("we've got a cephaloproblem: len(p)=%v, len(op)=%v\n", len(problems), len(opLine))
	}
	for i, p := range problems {
		if op := opLine[i]; op == "*" {
			p.operator = mul
		} else if op == "+" {
			p.operator = add
		} else {
			log.Fatalf("failed to parse op, unexpected op: %v", op)
		}
	}

	return problems
}
