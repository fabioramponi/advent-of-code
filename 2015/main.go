package main

import (
	"advent-of-code/2015/utils"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

var solutions map[int]func([]string) []string = map[int]func([]string) []string{
	1: day01,
	2: day02,
	3: day03,
	4: day04,
	5: day05,
}

func main() {
	if len(os.Args) > 1 {
		day, err := strconv.Atoi(os.Args[1])
		utils.Check(err)
		fmt.Println(solveDay(day))
	} else {
		keys := make([]int, 0, len(solutions))
		for k := range solutions {
			keys = append(keys, k)
		}
		sort.Ints(keys)
		for _, i := range keys {
			fmt.Printf("\nDay %d\n", i)
			fmt.Println(solveDay(i))
		}
	}
}

func solveDay(day int) string {
	input_file := fmt.Sprintf("inputs/day_%02d.txt", day)
	return strings.Join(solutions[day](utils.ReadInput(input_file)), "\n")
}
