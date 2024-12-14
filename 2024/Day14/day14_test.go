package Day14

import (
	i "image"
	"strings"
	"testing"
)

func TestGetBots(t *testing.T) {
	input := `p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2`
	exp := []Bot{
		{
			i.Point{X: 0, Y: 4},
			i.Point{X: 3, Y: -3},
		},
		{
			i.Point{X: 6, Y: 3},
			i.Point{X: -1, Y: -3},
		},
		{
			i.Point{X: 10, Y: 3},
			i.Point{X: -1, Y: 2},
		},
	}

	lines := strings.Split(input, "\n")

	bots := getBots(lines)
	if len(bots) != 3 {
		t.Errorf("Not enough bots, expected %v, got %v\n", 3, len(bots))
	}
	if *bots[0] != exp[0] {
		t.Errorf("Lists aren't equal, expected %+v, got %+v\n", exp, bots)
	}
	if *bots[1] != exp[1] {
		t.Errorf("Lists aren't equal, expected %+v, got %+v\n", exp, bots)
	}
	if *bots[2] != exp[2] {
		t.Errorf("Lists aren't equal, expected %+v, got %+v\n", exp, bots)
	}
}

// set constmaxx 11 and set constmaxy 7
//func TestBotMove(t *testing.T) {
//	// p=2,4 v=2,-3
//	bot := Bot{Loc: i.Pt(2, 4), Vel: i.Pt(2, -3)}
//	locations := []i.Point{
//		{
//			4, 1,
//		},
//		{
//			6, 5,
//		},
//		{
//			8, 2,
//		},
//		{
//			10, 6,
//		},
//		{
//			1, 3,
//		},
//	}
//	for x := range locations {
//		bot.move()
//		if !bot.Loc.Eq(locations[x]) {
//			t.Errorf("Bot not in right spot @ iteration %v. Expected %+v, got %+v", x, locations[x], bot.Loc)
//		}
//	}
//
//	bot = Bot{Loc: i.Pt(2, 4), Vel: i.Pt(2, -3)}
//	bot.moveForSeconds(3)
//	if !bot.Loc.Eq(locations[2]) {
//		t.Errorf("Bot not in right spot @ iteration %v. Expected %+v, got %+v", 3, locations[2], bot.Loc)
//	}
//
//}
//
//// set constmaxx 11 and set constmaxy 7
//func TestGetPart1(t *testing.T) {
//	input := `p=0,4 v=3,-3
//p=6,3 v=-1,-3
//p=10,3 v=-1,2
//p=2,0 v=2,-1
//p=0,0 v=1,3
//p=3,0 v=-2,-2
//p=7,6 v=-1,-3
//p=3,0 v=-1,-2
//p=9,3 v=2,3
//p=7,3 v=-1,2
//p=2,4 v=2,-3
//p=9,5 v=-3,-3`
//	lines := strings.Split(input, "\n")
//	res := getPart1(lines)
//	if res != 12 {
//		t.Errorf("Didn't get expected, got %v", res)
//	}
//}
