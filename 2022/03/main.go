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
}

func part1(lines []string) int {

    return 0
}
