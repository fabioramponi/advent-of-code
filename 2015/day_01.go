package main

import (
	"fmt"
	"strings"
)

func day01(input []string) []string {

	res := make([]string, 2)

	open_parentheses := 0
	total_parentheses := 0
	for _, l := range input {
		open_parentheses += strings.Count(l, "(")
		total_parentheses += len(l)
	}
	res[0] = fmt.Sprintf("answer_1: %d", 2*open_parentheses-total_parentheses)

	floor := 0
	var instructions int
	for i, c := range input[0] {
		if c == '(' {
			floor += 1
		} else {
			floor -= 1
		}

		if floor == -1 {
			instructions = i + 1
			break
		}
	}
	res[1] = fmt.Sprintf("answer_2: %d", instructions)

	return res
}
