package main

import (
	"fmt"
)

func countChars(s string) (int, int, int) {
	charLen := len(s)
	notRepr := 2
	newCharsForEncoding := 2
	for i := 0; i < len(s); i++ {
		if s[i] == '\\' {
			if s[i+1] == '"' || s[i+1] == '\\' {
				notRepr += 1
				i += 1
			} else if s[i+1] == 'x' {
				notRepr += 3
				i += 3
			}
		}
	}

	for _, r := range s {
		switch r {
		case '\\':
			newCharsForEncoding += 1
		case '"':
			newCharsForEncoding += 1
		}
	}

	res := charLen - notRepr
	return charLen, res, newCharsForEncoding
}

func day08(input []string) []string {
	res1 := 0
	res2 := 0
	for _, s := range input {
		literal, chars, additionalCharsForEncoding := countChars(s)
		fmt.Println(s, literal, chars)
		res1 += literal - chars
		res2 += additionalCharsForEncoding
	}

	return []string{fmt.Sprintf("answer_1: %d", res1), fmt.Sprintf("answer_2: %d", res2)}
}
