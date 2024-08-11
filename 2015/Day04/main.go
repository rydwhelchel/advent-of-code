package main

import (
	"crypto"
	_ "crypto/md5"
	"encoding/hex"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

func main() {
	inputFile := "./input.txt"
	data, err := os.ReadFile(inputFile)
	if err != nil {
		log.Panicln("Failed to read file")
	}

	data = []byte(strings.Fields(string(data))[0])
	println(data)

	t := time.Now()
	log.Printf("Part 1: %v\n", part1(data))
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
	t = time.Now()
	log.Printf("Part 2: %v\n", part2(data))
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
}

type HashedMod struct {
	mod  int
	hash []byte
}

func part1(data []byte) int {
	for i := 0; i < 10_000_000; i++ {
		byted := []byte(strconv.Itoa(i))
		y := append(data, byted...)
		x := createMD5(y)
		allZeroes := true
		if len(x) >= 5 {
			x := hex.EncodeToString(x)
			for _, v := range x[0:5] {
				if v != '0' {
					allZeroes = false
					break
				}
			}
			if allZeroes {
				return i
			}
		}

	}
	return 0
}

func createMD5(data []byte) []byte {
	hash := crypto.MD5.New()
	io.WriteString(hash, string(data))
	return hash.Sum(nil)
}

// Go is heckin quick just brute force it :)
func part2(data []byte) int {
	for i := 0; i < 10_000_000; i++ {
		byted := []byte(strconv.Itoa(i))
		y := append(data, byted...)
		x := createMD5(y)
		allZeroes := true
		if len(x) >= 6 {
			x := hex.EncodeToString(x)
			for _, v := range x[0:6] {
				if v != '0' {
					allZeroes = false
					break
				}
			}
			if allZeroes {
				return i
			}
		}

	}
	return 0
}
