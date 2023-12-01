package main

import (
	"bufio"
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	part1()
	part2()
}

func part1() {
	file, err := os.Open("./input.txt")
	check(err)
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	score := 0
	for scanner.Scan() {
		line := scanner.Text()
		a, b := line[0], line[2]
		switch b {
		case 'X':
			score += 1
			switch a {
			case 'A':
				score += 3
			case 'C':
				score += 6
			}
		case 'Y':
			score += 2
			switch a {
			case 'A':
				score += 6
			case 'B':
				score += 3
			}
		case 'Z':
			score += 3
			switch a {
			case 'B':
				score += 6
			case 'C':
				score += 3
			}
		}
	}

	fmt.Printf("Final score is %d\n", score)
}

func part2() {
	file, err := os.Open("./input.txt")
	check(err)
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	score := 0
	for scanner.Scan() {
		line := scanner.Text()
		a, b := line[0], line[2]
		switch b {
		case 'X': //Lose
			switch a {
			case 'A':
				score += 3
			case 'B':
				score += 1
			case 'C':
				score += 2
			}
		case 'Y': //Draw
			score += 3
			switch a {
			case 'A':
				score += 1
			case 'B':
				score += 2
			case 'C':
				score += 3
			}
		case 'Z': //Win
			score += 6
			switch a {
			case 'A':
				score += 2
			case 'B':
				score += 3
			case 'C':
				score += 1
			}
		}
	}

	fmt.Printf("Final score is %d\n", score)
}
