package main

import (
	"testing"
)

func TestDay08_0(t *testing.T) {
	input := "\"\""
	literal, nChars, newChars := countChars(input)
	if literal != 2 {
		t.Errorf("literal = %d; want 2", literal)
	}
	if nChars != 0 {
		t.Errorf("nChars = %d; want 0", nChars)
	}
	if newChars != 4 {
		t.Errorf("newChars = %d; want 4", newChars)
	}
}

func TestDay08_1(t *testing.T) {
	input := "\"abc\""
	literal, nChars, newChars := countChars(input)
	if literal != 5 {
		t.Errorf("literal = %d; want 5", literal)
	}
	if nChars != 3 {
		t.Errorf("nChars = %d; want 3", nChars)
	}
	if newChars != 4 {
		t.Errorf("newChars = %d; want 4", newChars)
	}
}

func TestDay08_2(t *testing.T) {
	input := "\"aaa\\\"aaa\""
	literal, nChars, newChars := countChars(input)
	if literal != 10 {
		t.Errorf("literal = %d; want 10", literal)
	}
	if nChars != 7 {
		t.Errorf("nChars = %d; want 7", nChars)
	}
	if newChars != 6 {
		t.Errorf("newChars = %d; want 6", newChars)
	}
}

func TestDay08_3(t *testing.T) {
	input := "\"\\x27\""
	literal, nChars, newChars := countChars(input)
	if literal != 6 {
		t.Errorf("literal = %d; want 6", literal)
	}
	if nChars != 1 {
		t.Errorf("nChars = %d; want 1", nChars)
	}
	if newChars != 5 {
		t.Errorf("newChars = %d; want 5", newChars)
	}

}

func TestDay08_4(t *testing.T) {
	inputs := []string{"\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""}
	r := day08(inputs)
	if r[0] != "answer_1: 12" {
		t.Errorf("answer is '%s'; want 12", r[0])
	}
	if r[1] != "answer_2: 19" {
		t.Errorf("answer is '%s'; want 19", r[1])
	}
}
