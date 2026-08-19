package main

import (
	"fmt"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type content struct {
	header  string
	bullets []bullet
}

type bullet struct {
	date string
	text string
}

func createFirst() {
	//Unimplemented
}

func main() {

	// Read the current directory (".")
	entries, err := os.ReadDir("/Users/kkulis/Documents/atg/chrono/")
	fileCount := len(entries)
	check(err)

	if fileCount > 0 {
		fmt.Println("Files found")
		fmt.Println(strconv.Itoa(fileCount))

	}
	// if err != nil {
	// 	log.Fatal(err)
	// }
	//
	// for _, entry := range entries {
	// 	// Output the name and whether it's a directory
	// 	fmt.Printf("Name: %s | IsDir: %t\n", entry.Name(), entry.IsDir())
	// }

	path := "/Users/kkulis/Documents/atg/chrono/chronicle-2026-08-18.md"
	dat, err := os.ReadFile(path)
	check(err)

	fmt.Println(dat)
	// f, err := os.Open(path)
	// check(err)

	// fmt.Print(string(dat))
	// fmt.Println("Test")
}
