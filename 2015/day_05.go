package main

import (
	"fmt"
	"strings"
)

func check3Vowels(s *string) bool {
	vowels := []rune{'a', 'e', 'i', 'o', 'u'}
	count := 0
	for _, r := range *s {
		if contains(&vowels, r) {
			count++
		}
		if count == 3 {
			return true
		}
	}
	return false
}

func checkNotContains(s *string) bool {
	substrings := []string{"ab", "cd", "pq", "xy"}
	for _, ss := range substrings {
		if strings.Contains(*s, ss) {
			return false
		}
	}
	return true
}

func checkSameLetterTwice(s *string) bool {
	var prev rune
	for _, r := range *s {
		if r == prev {
			return true
		}
		prev = r
	}
	return false
}

func checkPairRepeats(s *string) bool {
	ss := *s
	for i := 0; i < len(ss)-3; i++ {
		if strings.Contains(ss[i+2:], ss[i:i+2]) {
			return true
		}
	}
	return false
}

func checkLetterRepeatsWithOneInBetween(s *string) bool {
	var secondLast rune
	var last rune
	for i, r := range *s {
		if i > 1 {
			if r == secondLast {
				return true
			}
		}
		secondLast = last
		last = r
	}
	return false

}

func contains(runes *[]rune, r rune) bool {
	for _, tr := range *runes {
		if r == tr {
			return true
		}
	}
	return false
}

func day05(input []string) []string {
	count1 := 0
	count2 := 0
	for _, s := range input {
		if check3Vowels(&s) && checkNotContains(&s) && checkSameLetterTwice(&s) {
			count1++
		}
		if checkPairRepeats(&s) && checkLetterRepeatsWithOneInBetween(&s) {
			count2++
		}
	}
	return []string{fmt.Sprintf("answer_1: %d", count1), fmt.Sprintf("answer_2: %d", count2)}
}
