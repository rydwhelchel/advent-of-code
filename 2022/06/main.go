package main

import (
	"bufio"
	"log"
	"os"
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
    log.Printf("part2: %v\n", part2)
}

func part1(lines []string) int {
    last4 := []string{}
    // Only 1 line
    for i, c := range lines[0] {
        if len(last4) == 3 {
            if contains(last4, string(c)) || hasDupe(last4) {
                last4 = append(last4[1:], string(c))
            } else {
                return i + 1
            }
        } else {
            last4 = append(last4, string(c))
        }
    }

    return -1
}

func part2(lines []string) int {
    last4 := []string{}
    // Only 1 line
    for i, c := range lines[0] {
        if len(last4) == 13 {
            if contains(last4, string(c)) || hasDupe(last4) {
                last4 = append(last4[1:], string(c))
            } else {
                return i + 1
            }
        } else {
            last4 = append(last4, string(c))
        }
    }

    return -1
}

func contains(str []string, check string) bool {
    for _, c := range str {
        if string(c) == check {
            return true
        }
    }
    return false
}

func hasDupe(str []string) bool {
    for i := range str {
        for j := i + 1; j < len(str); j++ {
            if str[i] == str[j] {
                return true
            }
        }
    }

    return false
}
