package Day05

import (
	helpers "github.com/rydwhelchel/advent-of-code/helpers/go"
	"slices"
	"strconv"
	"strings"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 5" }

func (ctx *Context) Part1() string {
	lines := helpers.ReadInputAsString(ctx.Path)
	// top half & bottom half
	parts := strings.Split(lines, "\n\n")
	var rules [][]string
	rawRules := strings.Split(parts[0], "\n")
	for _, rule := range rawRules {
		rules = append(rules, strings.Split(rule, "|"))
	}
	cfCtx := compFuncCtx{rules: rules}

	sum := 0
	pgNumSets := strings.Split(parts[1], "\n")
	for _, pgNums := range pgNumSets {
		nums := strings.Split(pgNums, ",")
		// new Go feature! yay release notes
		if slices.IsSortedFunc(nums, cfCtx.compFunc) {
			val, _ := strconv.Atoi(nums[len(nums)/2])
			sum += val
		}
	}

	return strconv.Itoa(sum)
}

func (ctx *Context) Part2() string {
	lines := helpers.ReadInputAsString(ctx.Path)
	// top half & bottom half
	parts := strings.Split(lines, "\n\n")
	var rules [][]string
	rawRules := strings.Split(parts[0], "\n")
	for _, rule := range rawRules {
		rules = append(rules, strings.Split(rule, "|"))
	}
	cfCtx := compFuncCtx{rules: rules}

	sum := 0
	pgNumSets := strings.Split(parts[1], "\n")
	for _, pgNums := range pgNumSets {
		nums := strings.Split(pgNums, ",")
		if !slices.IsSortedFunc(nums, cfCtx.compFunc) {
			slices.SortFunc(nums, cfCtx.compFunc)
			val, _ := strconv.Atoi(nums[len(nums)/2])
			sum += val
		}
	}

	return strconv.Itoa(sum)
}

type compFuncCtx struct {
	rules [][]string
}

// compFunc needs access to rules, so built a context
func (ctx *compFuncCtx) compFunc(a, b string) int {
	for _, rule := range ctx.rules {
		// -1 is if a < b
		if a == rule[0] && b == rule[1] {
			return -1
		}
	}
	// not sorted
	return 1
}
