package main

import (
	"encoding/json"
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

type ChronoMarkdownFile struct {
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
	bullets []Entry
}

// .chro file extension
type ChronoFile struct {
	Logs []Log `json:"logs"`
}

type Log struct {
	Date    string  `json:"date"`
	Entries []Entry `json:"entries"`
}

type Entry struct {
	Time string `json:"time"`
	Text string `json:"text"`
}

func createFirst(fileName string) {
	//Unimplemented
	fmt.Println(fileName)
}

func main() {

	now := time.Now()
	nowF := now.Format("2006-01-02")
	// year, month, day := now.Date()

	log1 := Log{
		Date: nowF,
		Entries: []Entry{
			{
				Time: "11:00",
				Text: "Blah blah blah",
			},
		},
	}

	b, _ := json.Marshal(log1)

	os.WriteFile("test.chro", b, 0666)

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
