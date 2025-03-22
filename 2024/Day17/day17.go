package Day17

import (
	"encoding/json"
	h "github.com/rydwhelchel/advent-of-code/helpers/go"
	"log"
	"math"
	"strconv"
	"strings"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 17" }

func (ctx *Context) WIP() string { return "Part 2 WIP" }

type State struct {
	A, B, C      int
	instructions []int
	insPointer   int
	out          []string
}

func (ctx *Context) Part1() string {
	state := initializeState(h.ReadInputAsString(ctx.Path))

	for state.insPointer < len(state.instructions)-1 {
		state.performOp()
	}

	return strings.Join(state.out, ",")
}

func (ctx *Context) Part2() string {
	state := initializeState(h.ReadInputAsString(ctx.Path))

	for i := 0; toStr(state.instructions) != strings.Join(state.out, ","); i++ {
		state.A = i
		state.insPointer = 0
		for state.insPointer < len(state.instructions)-1 {
			state.performOp()
		}
	}

	return strings.Join(state.out, ",")
}

func toStr(ints []int) string {
	var out []string
	for _, i := range ints {
		val := strconv.Itoa(i)
		out = append(out, val)
	}
	return strings.Join(out, ",")
}

func initializeState(input string) State {
	lines := strings.Split(input, "\n")
	a, _ := strconv.Atoi(strings.Split(lines[0], ": ")[1])
	b, _ := strconv.Atoi(strings.Split(lines[1], ": ")[1])
	c, _ := strconv.Atoi(strings.Split(lines[2], ": ")[1])
	var ins []int
	_ = json.Unmarshal([]byte("["+strings.Split(lines[4], ": ")[1]+"]"), &ins)
	return State{
		A:            a,
		B:            b,
		C:            c,
		instructions: ins,
		insPointer:   0,
		out:          make([]string, 0),
	}
}

func (s *State) reg(x int) int {
	if x >= 0 && x < 4 {
		return x
	}
	if x == 4 {
		return s.A
	}
	if x == 5 {
		return s.B
	}
	if x == 6 {
		return s.C
	}
	log.Panicf("THIS IS NOT A VALID PROGRAM, %v\n", x)
	// unreachable, but goland doesn't know that
	return 0
}

// op0  adv x -> A/2**reg(x) -> A
// op1  bxl x -> B^x -> B   		(literal x)
// op2  bst x -> B%8 -> B   		(only keeps 3 lowest bits)
// op3  jnz x -> A!=0? insPointer = x (do not jump 2 after, literal x)
// op4  bxc _ -> B^C -> B
// op5  out x -> reg(x)%8 ->>
// op6  adv x -> A/2**reg(x) -> B
// op7  adv x -> A/2**reg(x) -> C
func (s *State) performOp() {
	opCode := s.instructions[s.insPointer]
	operand := s.instructions[s.insPointer+1]
	switch opCode {
	case 0:
		s.A = int(float64(s.A) / math.Pow(2, float64(s.reg(operand))))
	case 1:
		s.B = s.B ^ operand
	case 2:
		s.B = s.reg(operand) % 8
	case 3:
		if s.A != 0 {
			s.insPointer = operand
			s.insPointer -= 2
		}
	case 4:
		s.B = s.B ^ s.C
	case 5:
		s.out = append(s.out, strconv.Itoa(s.reg(operand)%8))
	case 6:
		s.B = int(float64(s.A) / math.Pow(2, float64(s.reg(operand))))
	case 7:
		s.C = int(float64(s.A) / math.Pow(2, float64(s.reg(operand))))
	}
	s.insPointer += 2
}
