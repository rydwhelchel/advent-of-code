package main

import (
	"bufio"
	"log"
	"os"
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
    log.Printf("Part1: %v\n", part1)
    part2 := part2(lines)
    log.Printf("Part2: %v\n", part2)
}

func part1(lines []string) int {
    sum := 0

    for _, line := range lines {
        midmark := len(line)/2
        firstHalf := line[0:midmark]
        secondHalf := line[midmark:]
        for _, char := range firstHalf {
            if strings.Contains(secondHalf, string(char)) {
                sum += calcValue(char)
                break
            } 
        }

    }

    return sum
}

func part2(lines []string) int {
    sum := 0

    group := []string{}
    for _, line := range lines {
        if len(group) < 3 {
            group = append(group, line)
            if len(group) < 3 {
                continue
            }
        }
        firstLine := group[0]
        secondLine := group[1]
        thirdLine := group[2]
        for _, char := range firstLine {
            if strings.Contains(secondLine, string(char)) && strings.Contains(thirdLine, string(char)) {
                sum += calcValue(char)
                break
            }
        }
        group = []string{}
    }

    return sum
}


func calcValue(r rune) int {
    if r > 96 {
        return int(r) - 96
    }
    return int(r) - 64 + 26
}
