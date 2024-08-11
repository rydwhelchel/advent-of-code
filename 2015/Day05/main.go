package main

import (
	"log"
	"os"
	"strings"
	"time"
)

func main() {
	inputFile := "./input.txt"
	data, err := os.ReadFile(inputFile)
	if err != nil {
		log.Panicln("Failed to read file")
	}
	t := time.Now()
	log.Printf("Part 1: %v\n", part1(data))
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
	t = time.Now()
	log.Printf("Part 2: %v\n", part2(data))
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
}

// Nice string rules
// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
// It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
func rule1(s string) bool {
	vowels := map[rune]bool{
		'a': true,
		'e': true,
		'i': true,
		'o': true,
		'u': true,
	}
	num := 0
	for _, c := range s {
		if vowels[c] {
			num++
		}
		if num >= 3 {
			return true
		}
	}
	return false
}

func rule2(s string) bool {
	var prev rune
	for _, v := range s {
		if v == prev {
			return true
		}
		prev = v
	}
	return false
}

func rule3(s string) bool {
	forbidden := []string{"ab", "cd", "pq", "xy"}
	for _, sub := range forbidden {
		if strings.Contains(s, sub) {
			return false
		}
	}
	return true
}

func part1(data []byte) int {
	nice := 0
	lines := strings.Split(string(data), "\n")
	for _, v := range lines {
		if rule1(v) && rule2(v) && rule3(v) {
			nice++
		}
	}
	return nice
}

func part2(data []byte) int {
	nice := 0
	lines := strings.Split(string(data), "\n")
	for _, line := range lines {
		var nice1, nice2 bool
		for i := 0; i < len(line)-2; i++ {
			if nice1 || strings.Contains(line[i+2:], line[i:i+2]) {
				nice1 = true
			}
			if nice2 || line[i] == line[i+2] {
				nice2 = true
			}
			if nice1 && nice2 {
				nice++
				break
			}
		}
	}
	return nice
}
