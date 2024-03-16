package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
    file, err := os.Open("input")
    if err != nil {
        log.Println("Error opening file")
        return
    }

    sc := bufio.NewScanner(file)
    lines := []string{}
    for sc.Scan() {
        text := sc.Text()
        lines = append(lines, text)
    }
    part1 := part1(lines)
    fmt.Printf("Part1: %v\n", part1)
    part2 := part2(lines)
    fmt.Printf("part2: %v\n", part2)
}

func contains(lis []int, i int) bool {
    for _, v := range lis {
        if v == i {
            return true
        }
    }
    return false
}

func createStacks(lines []string) [][]string {
    stacks := make(map[int][]string)
    nonAlpha := "[] "
    vals := []int{}

    for j := range lines {
        // Iterates backwards through the lines to build the stacks
        line := lines[len(lines)-j-1]
        for i, c := range line {
            if !strings.Contains(nonAlpha, string(c)) {
                if !contains(vals, i) {
                    vals = append(vals, i)
                }
                stacks[i] = append(stacks[i], string(c))
            }
        }
    }

    s := [][]string{}
    s = append(s, []string{})
    for _, v := range vals {
        s = append(s, stacks[v])
    }

    return s
}

func insertAt(arr []string, v string, i int) []string {
    arr = append(arr[:i+1], arr[i:]...)
    arr[i] = v
    return arr
}

func pop(arr []string) ([]string, string) {
    return arr[:len(arr)-1], arr[len(arr)-1]
}

type Instruction struct {
    count, destination, source int
}

func parseInstruction(line string) Instruction {
    si := strings.Split(line, " ")
    count, _ := strconv.Atoi(si[1])
    source, _ := strconv.Atoi(si[3])
    destination, _ := strconv.Atoi(si[5])

    return Instruction {
        count: count,
        source: source,
        destination: destination,
    }
}

func part1(lines []string) string {
    stackLines := []string{}
    for _, line := range lines {
        if strings.Contains(line, "[") {
            stackLines = append(stackLines, line)
        } else {
            break;
        }
    }

    stacks := createStacks(stackLines)

    for _, l := range lines {
        if !strings.Contains(l, "move") {
            continue
        }
        ins := parseInstruction(l)

        // Amount of times to move values
        for i := 0; i < ins.count; i++ {
            if len(stacks[ins.source]) == 0 {
                continue
            }
            newStack, v := pop(stacks[ins.source])
            stacks[ins.destination] = append(stacks[ins.destination], v)
            stacks[ins.source] = newStack
        }
    }

    var str strings.Builder
    for _, s := range stacks {
        fmt.Printf("Stack %v\n", s)

        if len(s) > 0 {
            _, l := pop(s)
            str.WriteString(l)
        }
    }
    return str.String()
}

func part2(lines []string) string {
    stackLines := []string{}
    for _, line := range lines {
        if strings.Contains(line, "[") {
            stackLines = append(stackLines, line)
        } else {
            break;
        }
    }

    stacks := createStacks(stackLines)

    for _, l := range lines {
        if !strings.Contains(l, "move") {
            continue
        }
        ins := parseInstruction(l)

        // How did this actually work
        if ins.count > len(stacks[ins.source]) {
            stacks[ins.destination] = append(stacks[ins.destination], stacks[ins.source]...)
            stacks[ins.source] = []string{}
        } else {
            stacks[ins.destination] = append(stacks[ins.destination],
                stacks[ins.source][len(stacks[ins.source])-ins.count:]...)
            stacks[ins.source] = stacks[ins.source][:len(stacks[ins.source])-ins.count]
        }
    }

    var str strings.Builder
    for _, s := range stacks {
        fmt.Printf("Stack %v\n", s)

        if len(s) > 0 {
            _, l := pop(s)
            str.WriteString(l)
        }
    }
    return str.String()
}
