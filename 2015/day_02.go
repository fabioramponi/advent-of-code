package main

import (
	"advent-of-code/2015/utils"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type packSize struct {
	x int
	y int
	z int
}

func (pack packSize) neededPaper() int {
	a1 := pack.x * pack.y
	a2 := pack.y * pack.z
	a3 := pack.x * pack.z
	extra := utils.MinOf(a1, a2, a3)
	return 2*a1 + 2*a2 + 2*a3 + extra
}

func (pack packSize) volume() int {
	return pack.x * pack.y * pack.z
}

func (pack packSize) neededRibbon() int {
	arr := []int{pack.x, pack.y, pack.z}
	sort.Ints(arr)
	return 2*arr[0] + 2*arr[1] + pack.volume()
}

func day02(input []string) []string {
	res := []string{}
	totalPaper := 0
	totalRibbon := 0
	for _, l := range input {
		splitted := strings.Split(l, "x")
		x, _ := strconv.Atoi(splitted[0])
		y, _ := strconv.Atoi(splitted[1])
		z, _ := strconv.Atoi(splitted[2])
		pack := packSize{x, y, z}
		totalPaper += pack.neededPaper()
		totalRibbon += pack.neededRibbon()
	}
	res = append(res, fmt.Sprintf("answer_1: %d", totalPaper))
	res = append(res, fmt.Sprintf("answer_2: %d", totalRibbon))
	return res
}
