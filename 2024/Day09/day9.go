package Day09

import (
	h "github.com/rydwhelchel/advent-of-code/helpers/go"
	"slices"
	"strconv"
	"strings"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 9" }

func (ctx *Context) Part1() string {
	input := h.ReadInputAsString(ctx.Path)

	// val at each point is the index
	var fileSystem []int
	var curr = 0
	// build initial state of filesystem
	for i := 0; i < len(input); i++ {
		id := curr
		if i%2 == 1 {
			// empty
			id = -1
		} else {
			curr++
		}
		val, _ := strconv.Atoi(string(input[i]))

		fileSystem = append(fileSystem, slices.Repeat([]int{id}, val)...)
	}

	left, right := 0, len(fileSystem)-1
	sum := 0

	for left <= right {
		if fileSystem[left] == -1 {
			sum += fileSystem[right] * left
			right--
		} else {
			sum += fileSystem[left] * left
		}

		for fileSystem[right] == -1 {
			right--
		}
		left++
	}

	return strconv.Itoa(sum)
}

type memoryTakenBy int

const (
	EMPTY memoryTakenBy = iota
	FILE
)

type memory struct {
	memoryTakenBy memoryTakenBy
	space         int
	value         int
}

func (m *memory) String() string {
	if m.memoryTakenBy == FILE {
		return strings.Join(slices.Repeat([]string{strconv.Itoa(m.value)}, m.space), "")
	} else {
		return strings.Join(slices.Repeat([]string{"."}, m.space), "")
	}
}

func (ctx *Context) Part2() string {
	input := h.ReadInputAsString(ctx.Path)
	var fileSystem []memory

	curr := 0
	// build initial state of filesystem
	for i := 0; i < len(input); i++ {
		takenBy := FILE
		id := curr
		if i%2 == 1 {
			// empty
			id = -1
			takenBy = EMPTY
		} else {
			curr++
		}
		val, _ := strconv.Atoi(string(input[i]))

		fileSystem = append(fileSystem, memory{
			memoryTakenBy: takenBy,
			space:         val,
			value:         id,
		})
	}

	//fmt.Println("~~~ Filesystem before ~~~")
	//for _, f := range fileSystem {
	//	fmt.Printf("%v", f.String())
	//}
	//fmt.Print("\n\n")

	// move files over
	left, right := 0, len(fileSystem)-1
	for right > 0 {
		for left < right {
			leftF := fileSystem[left]
			rightF := fileSystem[right]
			if leftF.memoryTakenBy == EMPTY {
				if leftF.space >= rightF.space {
					// try refactoring to use slices.Insert
					leftSide, rightSide := fileSystem[:left], fileSystem[left+1:]
					if leftF.space-rightF.space > 0 {
						rightSide = append([]memory{{EMPTY, leftF.space - rightF.space, -1}}, rightSide...)
						// adjust right pointer to account for additional file in list
						right++
					}
					leftSide = append(leftSide, rightF)
					fileSystem = append(leftSide, rightSide...)
					// sloppy handling here, should refactor if I care
					fileSystem[right].memoryTakenBy = EMPTY
					fileSystem[right].value = -1
					break
				}
			}
			left++
		}
		left = 0
		right--
		for fileSystem[right].memoryTakenBy == EMPTY {
			right--
		}
	}

	//fmt.Println("~~~ Filesystem after ~~~")
	//for _, f := range fileSystem {
	//	fmt.Printf("%v", f.String())
	//}
	//fmt.Print("\n\n")

	sum, index := 0, 0
	for _, mem := range fileSystem {
		for j := 0; j < mem.space; j++ {
			// Empty has a 0 value on it so I can shoot from the hip here
			sum += index * int(mem.memoryTakenBy) * mem.value
			index++
		}
	}

	return strconv.Itoa(sum)
}
