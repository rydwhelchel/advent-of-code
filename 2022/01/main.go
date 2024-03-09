package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
)

func handleErr(err error) {
    error := log.New(os.Stderr, "ERROR: ", log.LstdFlags)
    error.Printf("%v\n", err)
}

func main() {
    file, err := os.Open("input")
    if err != nil {
        handleErr(err)
        return
    }

    part1 := part1(file)

    log.Printf("Part1: %v\n", part1)
    file.Close()

    file, err = os.Open("input")
    if err != nil {
        handleErr(err)
        return
    }

    defer file.Close()

    part2 := part2(file)

    log.Printf("Part2: %v\n", part2)
}


func part1(file *os.File) int {
    sc := bufio.NewScanner(file)
    currmax := 0
    curr := 0
    for sc.Scan() {
        text := sc.Text()
        if text == "" {
            if currmax < curr {
                currmax = curr
            }
            curr = 0
            continue
        }

        block, err := strconv.Atoi(sc.Text())
        if err != nil {
            handleErr(err)
            return currmax
        }
        curr += block
    }
    return currmax
}

func sum(i []int) int {
    c := 0
    for _, v := range i {
        c += v
    }

    return c
}

func sortDesc(l []int) {
    sort.Slice(l, func(i, j int) bool {
        return l[i] < l[j]
    })
}

func part2(file *os.File) int {
    sc := bufio.NewScanner(file)
    currmaxes := []int{}
    curr := 0
    for sc.Scan() {
        text := sc.Text()
        if text == "" {
            log.Printf("Currmax is: %v\n", currmaxes)
            if len(currmaxes) < 3 {
                currmaxes = append(currmaxes, curr)
            } else {
                for i, c := range currmaxes {
                    if c < curr {
                        currmaxes[i] = curr
                        break
                    }
                } 
            }
            sortDesc(currmaxes)
            curr = 0
            continue
        }

        block, err := strconv.Atoi(sc.Text())
        if err != nil {
            handleErr(err)
            return sum(currmaxes)
        }
        curr += block
    }
    return sum(currmaxes)
}
