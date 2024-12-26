package Day13

import (
	"encoding/json"
	h "github.com/rydwhelchel/advent-of-code/helpers/go"
	"log"
	"strconv"
	"strings"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 13" }

// x = prize x location
// y = prize y location
// x1 = distance moved in X by pressing A
// y1 = distance moved in Y by pressing A
// x2 = distance moved in X by pressing B
// x2 = distance moved in X by pressing B

// x1*A + x2*B = x
// y1*A + y2*B = y

// A = (x - x2*B)/x1
// B = (y - y1*A)/y2

// plugging in A from above
// y1*A + y2*B = y
// y1*((x-x2*B)/x1) + y2*B = y       spread out y1 then multiply all by x1
// x*y1 - x2*B*y1 + x1*y2*B = y*x1
// x1*y2*B - x2*y1*B = y*x1 - x*y1
// B(x1*y2 - x2*y1) = y*x1 - x*y1
// B = (y*x1 - x*y1) / (x1*y2 - x2*y1)

// plugging in B from above
// x1*A + x2*B = x
// x1*A + x2*((y-y1*A)/y2) = x
// x1*A + (x2*y - x2*y1*A)/y2 = x
// x1*y2*A + x2*y - x2*y1*A = x*y2
// A(x1*y2 - x2*y1) = x*y2 - x2*y
// A = (x*y2 - x2*y)/(x1*y2 - x2*y1)

// Solutions
// A = (x*y2 - x2*y) / (x1*y2 - x2*y1)
// B = (y*x1 - x*y1) / (x1*y2 - x2*y1)

// testing
// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
// I think it has to be a whole number or else there is no solution
// A = (8400*67 - 22*5400) / (94*67 - 22*34) = 80
// B = (5400*94 - 8400*34) / (94*67 - 22*34) = 40
// cost = 80*3 + 40 = 280
// Checks out!!!

type game struct {
	aX, aY, bX, bY, prizeX, prizeY int
}

func (ctx *Context) Part1() string {
	// A = (x*y2 - x2*y) / (x1*y2 - x2*y1)
	// B = (y*x1 - x*y1) / (x1*y2 - x2*y1)
	input := h.ReadInputAsString(ctx.Path)
	games := parseGames(input)

	sum := 0
	for _, game := range games {
		var x, y, x1, y1, x2, y2 float64
		x, y, x1, y1, x2, y2 = float64(game.prizeX), float64(game.prizeY), float64(game.aX), float64(game.aY), float64(game.bX), float64(game.bY)
		A := (x*y2 - x2*y) / (x1*y2 - x2*y1)
		B := (y*x1 - x*y1) / (x1*y2 - x2*y1)
		// If either number is not a whole number then the answer is invalid
		if A != float64(int(A)) || B != float64(int(B)) {
			continue
		}
		sum += int(A)*3 + int(B)
	}

	return strconv.Itoa(sum)
}

// Part2 is identical to Part1 except it adds 10000000000000 offset to the prize
func (ctx *Context) Part2() string {
	input := h.ReadInputAsString(ctx.Path)
	games := parseGames(input)

	sum := 0
	for _, game := range games {
		var x, y, x1, y1, x2, y2 float64
		x, y, x1, y1, x2, y2 = float64(game.prizeX), float64(game.prizeY), float64(game.aX), float64(game.aY), float64(game.bX), float64(game.bY)
		x += 10000000000000
		y += 10000000000000
		A := (x*y2 - x2*y) / (x1*y2 - x2*y1)
		B := (y*x1 - x*y1) / (x1*y2 - x2*y1)
		// If either number is not a whole number then the answer is invalid
		if A != float64(int(A)) || B != float64(int(B)) {
			continue
		}
		sum += int(A)*3 + int(B)
	}

	return strconv.Itoa(sum)
}

const (
	ButtonAPrefix = "Button A: X+"
	ButtonBPrefix = "Button B: X+"
	ButtonYPrefix = " Y+"
	PrizeXPrefix  = "Prize: X="
	PrizeYPrefix  = "Y="
)

func parseGames(input string) []game {
	parts := strings.Split(input, "\n\n")

	var games []game
	for _, part := range parts {
		lines := strings.Split(part, "\n")
		if len(lines) != 3 {
			log.Println("WE DONE")
			break
		}
		var a, b, p []int
		// convert Button A: X+#, Y+# -> [#,#]
		aLine := strings.Replace(lines[0], ButtonAPrefix, "[", 1)
		aLine = strings.Replace(aLine, ButtonYPrefix, "", 1) + "]"
		_ = json.Unmarshal([]byte(aLine), &a)
		bLine := strings.Replace(lines[1], ButtonBPrefix, "[", 1)
		bLine = strings.Replace(bLine, ButtonYPrefix, "", 1) + "]"
		_ = json.Unmarshal([]byte(bLine), &b)
		prizeLine := strings.Replace(lines[2], PrizeXPrefix, "[", 1)
		prizeLine = strings.Replace(prizeLine, PrizeYPrefix, "", 1) + "]"
		_ = json.Unmarshal([]byte(prizeLine), &p)

		games = append(games, game{
			aX:     a[0],
			aY:     a[1],
			bX:     b[0],
			bY:     b[1],
			prizeX: p[0],
			prizeY: p[1],
		})
	}

	return games
}
