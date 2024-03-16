package main

import (
	"bufio"
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
    log.Printf("Part1: %v", part1)
    part2 := part2(lines)
    log.Printf("part2: %v", part2)
    
}

type Range struct {
    low, high int
}

func parseLine(line string) []Range {
    ss := strings.Split(line, ",")
    s1 := strings.Split(ss[0], "-")
    s10, _ := strconv.Atoi(s1[0])
    s11, _ := strconv.Atoi(s1[1])
    s2 := strings.Split(ss[1], "-")
    s20, _ := strconv.Atoi(s2[0])
    s21, _ := strconv.Atoi(s2[1])
    range1 := Range {
        low: s10,
        high: s11,
    }
    range2 := Range {
        low: s20,
        high: s21,
    }
    return []Range{range1, range2}
}

func oneContains(r1, r2 Range) bool {
    if r1.low == r2.low {
        return true
    } else if r1.low < r2.low {
        if r1.high >= r2.high {
            return true
        }
    } else {
        if r1.high <= r2.high {
            return true
        }
    }
    return false
}

func part1(lines []string) int {
    contained := 0

    for _, line := range lines {
        ranges := parseLine(line)
        if oneContains(ranges[0], ranges[1]) {
            contained += 1
        }
    }

    return contained
}

func overlaps(r1, r2 Range) bool {
    if r1.low == r2.low || r1.high == r2.high {
        return true
    } else if r1.low < r2.low {
        if r1.high >= r2.low {
            return true
        }
    } else {
        if r1.low <= r2.high {
            return true
        }
    }
    return false
}

func part2(lines []string) int {
    overlapping := 0

    for _, line := range lines {
        ranges := parseLine(line)
        if overlaps(ranges[0], ranges[1]) {
            overlapping += 1
        }
    }

    return overlapping
}


