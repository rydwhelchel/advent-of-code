package Day07

import (
	"encoding/json"
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
	"strconv"
	"strings"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 7" }

func (ctx *Context) Part1() string {
	lines := helpers.ReadInputAsLines(ctx.Path)

	total := 0
	for _, line := range lines {
		calib := parseLine(line)
		if calib.possibleCalibration() {
			total += calib.res
		}
	}

	return strconv.Itoa(total)
}

func (ctx *Context) Part2() string {
	lines := helpers.ReadInputAsLines(ctx.Path)

	total := 0
	for _, line := range lines {
		calib := parseLine(line)
		if calib.possibleCalibration2() {
			total += calib.res
		}
	}

	return strconv.Itoa(total)
}

type calibration struct {
	res      int
	operands []int
}

const (
	ADD = iota
	MUL
	CON
)

func parseLine(line string) calibration {
	var c calibration
	splitified := strings.Split(line, " ")
	result, err := strconv.Atoi(strings.Trim(splitified[0], ":"))
	if err != nil {
		panic("failed to get res")
	}
	c.res = result
	err = json.Unmarshal([]byte("["+strings.Join(splitified[1:], ",")+"]"), &c.operands)
	if err != nil {
		panic("failed to json unmarshal :(")
	}
	return c
}

func (c calibration) possibleCalibration() bool {
	// defaults to all ADD
	var operators = make([]int, len(c.operands)-1)

	for {
		total := c.operands[0]
		for i, op := range c.operands[1:] {
			switch operators[i] {
			case ADD:
				total += op
			case MUL:
				total *= op
			default:
				panic("not ready for that bruh")
			}
		}
		if total == c.res {
			return true
		}
		operators = getNextOperators(operators)
		if len(operators) == 0 {
			break
		}
	}

	return false
}

func (c calibration) possibleCalibration2() bool {
	// defaults to all ADD
	var operators = make([]int, len(c.operands)-1)

	for {
		total := c.operands[0]
		for i, op := range c.operands[1:] {
			switch operators[i] {
			case ADD:
				total += op
			case MUL:
				total *= op
			case CON:
				op1 := strconv.Itoa(total)
				op2 := strconv.Itoa(op)
				res, err := strconv.Atoi(op1 + op2)
				if err != nil {
					panic("really bad stuff")
				}
				total = res
			}
		}
		if total == c.res {
			return true
		}
		operators = getNextOperators2(operators)
		if len(operators) == 0 {
			break
		}
	}

	return false
}

func getNextOperators(operators []int) []int {
	carry := 1
	for i := len(operators) - 1; i >= 0; i-- {
		if carry != 1 {
			break
		}
		res := operators[i] + carry
		// shouldnt be possible
		if res == 0 {
			operators[i] = res
			carry = 0
		} else if res == 1 {
			operators[i] = 1
			carry = 0
		} else if res == 2 {
			operators[i] = 0
			carry = 1
		}
	}
	if carry == 1 {
		return []int{}
	}
	return operators
}

func getNextOperators2(operators []int) []int {
	carry := 1
	for i := len(operators) - 1; i >= 0; i-- {
		if carry != 1 {
			break
		}
		res := operators[i] + carry
		// tedious but helps me visualize
		if res == 0 {
			operators[i] = res
			carry = 0
		} else if res == 1 {
			operators[i] = 1
			carry = 0
		} else if res == 2 {
			operators[i] = 2
			carry = 0
		} else if res == 3 {
			operators[i] = 0
			carry = 1
		}
	}
	if carry == 1 {
		return []int{}
	}
	return operators
}
