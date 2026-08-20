package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type chronoFile struct {
	name        string
	frontmatter frontmatter
	content     content
}

type frontmatter struct {
	prev    string
	journal string
	next    string
}

type content struct {
	header  string
	bullets []bullet
}

type bullet struct {
	timestamp string
	text      string
}

func createFirst(fileName string) {
	//Unimplemented
	fmt.Println(fileName)
}

func main() {

	now := time.Now()
	// year, month, day := now.Date()

	nowF := now.Format("2006-01-02")

	fmt.Println(nowF)

	// Read the current directory (".")
	entries, err := os.ReadDir("/Users/kkulis/Documents/atg/chrono/")
	fileCount := len(entries)
	check(err)

	if fileCount > 0 {
		fmt.Println("Files found")
		fmt.Println(strconv.Itoa(fileCount))

		fileName := "chronicle-" + nowF + ".md"
		createFirst(fileName)

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
