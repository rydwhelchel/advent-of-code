package day7

import (
	"errors"
	"log"
	"strconv"
	"strings"

	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
)

type Day7Context struct {
	Path string
}

func (ctx *Day7Context) Name() string {
	return "Day 7"
}

// I can't believe this worked first try, all those asserts for nada
func (ctx *Day7Context) Part1() string {
	lines := helpers.ReadInputAsLines(ctx.Path)
	return solvePart1(lines)
}

func (ctx *Day7Context) Part2() string {
	lines := helpers.ReadInputAsLines(ctx.Path)
	part1Output := solvePart1(lines)
	instructions := parseInstructions(lines)

	// override the value of b with wire a's output
	for i, instr := range instructions {
		if instr.outlet == "b" {
			instructions[i] = instruction{
				operands:  []string{part1Output},
				operation: INPUT,
				outlet:    "b",
			}
		}
	}

	vals := executeInstructions(instructions)
	// get the new value of 'a'
	return strconv.Itoa(vals["a"])
}

// Separating this out to make it testable
// didn't need to test it *shrug*
func solvePart1(lines []string) string {
	instructions := parseInstructions(lines)
	vals := executeInstructions(instructions)
	return strconv.Itoa(vals["a"])
}

type instruction struct {
	// Either 1 length or 2 length, can be a number or a wire
	operands  []string
	operation Operation
	// Only wires
	outlet string
}

type Operation int

const (
	INPUT Operation = iota
	LSHIFT
	RSHIFT
	AND
	OR
	NOT
)

// Builds a list of instructions from lines
func parseInstructions(lines []string) []instruction {
	instructions := []instruction{}

	for _, line := range lines {
		sides := strings.Split(line, " -> ")
		// end parsing
		if len(sides) < 2 {
			break
		}

		leftSide := sides[0]
		outlet := sides[1]

		chunks := strings.Split(leftSide, " ")

		// Putting a value directly on a wire
		if len(chunks) == 1 {
			instructions = append(instructions,
				instruction{
					operands:  []string{chunks[0]},
					operation: INPUT,
					outlet:    outlet,
				})
			continue
		}

		// NOT operand guaranteed
		if len(chunks) == 2 {
			if chunks[0] != "NOT" {
				panic("NOT operand not found when expected")
			}
			instructions = append(instructions,
				instruction{
					operands:  []string{chunks[1]},
					operation: NOT,
					outlet:    outlet,
				})
			continue
		}
		switch chunks[1] {
		case "LSHIFT":
			instructions = append(instructions,
				instruction{
					operands:  []string{chunks[0], chunks[2]},
					operation: LSHIFT,
					outlet:    outlet,
				})
		case "RSHIFT":
			instructions = append(instructions,
				instruction{
					operands:  []string{chunks[0], chunks[2]},
					operation: RSHIFT,
					outlet:    outlet,
				})
		case "AND":
			instructions = append(instructions,
				instruction{
					operands:  []string{chunks[0], chunks[2]},
					operation: AND,
					outlet:    outlet,
				})
		case "OR":
			instructions = append(instructions,
				instruction{
					operands:  []string{chunks[0], chunks[2]},
					operation: OR,
					outlet:    outlet,
				})
		}
	}

	return instructions
}

func executeInstructions(instructions []instruction) map[string]int {
	wireVals := map[string]int{}
	remainingInstructions := []instruction{}

	// Populate initial wires
	for _, instruction := range instructions {
		if instruction.operation == INPUT {
			if val, err := strconv.Atoi(instruction.operands[0]); err == nil {
				wireVals[instruction.outlet] = val
			} else {
				remainingInstructions = append(remainingInstructions, instruction)
			}
		} else {
			remainingInstructions = append(remainingInstructions, instruction)
		}
	}

	// Could end this early if we find the answer to the wire we're looking for ('a')
	for len(remainingInstructions) > 0 {
		nextRemainingInstructions := []instruction{}
		for _, instruction := range remainingInstructions {
			if ops, err := getOps(instruction, wireVals); err == nil {
				wireVals[instruction.outlet] = calcVal(ops, instruction.operation)
			} else {
				nextRemainingInstructions = append(nextRemainingInstructions, instruction)
			}
		}
		// verify this doesn't go wonky
		remainingInstructions = nextRemainingInstructions
	}

	return wireVals
}

func getOps(inst instruction, vals map[string]int) ([]int, error) {
	ops := []int{}

	for _, op := range inst.operands {
		if val, ok := vals[op]; ok {
			ops = append(ops, val)
		} else if val, err := strconv.Atoi(op); err == nil {
			ops = append(ops, val)
		} else {
			return nil, errors.New("Not an int")
		}
	}
	return ops, nil
}

func calcVal(ops []int, operation Operation) int {
	switch operation {
	case INPUT:
		if len(ops) != 1 {
			log.Panicf("Cannot INPUT ops not len 1")
		}
		return ops[0]
	case LSHIFT:
		if len(ops) != 2 {
			log.Panicf("Cannot LSHIFT ops not len 2")
		}
		return lshift(ops[0], ops[1])
	case RSHIFT:
		if len(ops) != 2 {
			log.Panicf("Cannot RSHIFT ops not len 2")
		}
		return rshift(ops[0], ops[1])
	case AND:
		if len(ops) != 2 {
			log.Panicf("Cannot AND ops not len 2")
		}
		return and(ops[0], ops[1])
	case OR:
		if len(ops) != 2 {
			log.Panicf("Cannot OR ops not len 2")
		}
		return or(ops[0], ops[1])
	case NOT:
		if len(ops) != 1 {
			log.Panicf("Cannot NOT ops not len 1")
		}
		return not(ops[0])
	default:
		log.Panicf("Unexpected operation")
		return -1
	}
}

func lshift(inp1, inp2 int) int {
	return inp1 << inp2
}

func rshift(inp1, inp2 int) int {
	return inp1 >> inp2
}

func and(inp1, inp2 int) int {
	return inp1 & inp2
}

func or(inp1, inp2 int) int {
	return inp1 | inp2
}

func not(inp int) int {
	return ^inp
}
