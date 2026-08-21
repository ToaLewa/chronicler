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

// Use .chro file extension if .json is too constraining
type ChronoFile struct {
	Logs []YearLog `json:"logs"`
}

type YearLog struct {
	Year   int        `json:"year"`
	Months []MonthLog `json:"months"`
}

type MonthLog struct {
	Month int      `json:"month"`
	Days  []DayLog `json:"days"`
}

type DayLog struct {
	Day     int     `json:"day"`
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

func hasArg() bool {
	return len(os.Args) > 1
}

const ChronoFileName = "chrono.json"

func main() {
	now := time.Now()
	dateString := now.Format("2006-01-02")
	hour, min, _ := time.Now().Local().Clock()
	timeString := fmt.Sprintf("%d:%02d", hour, min)
	year, month, day := now.Date()

	if hasArg() {
		userText := os.Args[1]
		oFile := ChronoFile{
			Logs: []YearLog{
				{
					Year: year,
					Months: []MonthLog{{
						Month: int(month),
						Days: []DayLog{
							{
								Day:  day,
								Date: dateString,
								Entries: []Entry{
									{
										Time: timeString,
										Text: userText,
									},
								},
							},
						},
					},
					},
				},
			},
		}

		b, _ := json.Marshal(oFile)

		os.WriteFile(ChronoFileName, b, 0666)

		fmt.Println("Write file")

	} else {
		fmt.Println(dateString)

		// Read the current directory (".")
		entries, err := os.ReadDir("/Users/kkulis/Documents/atg/chrono/")
		fileCount := len(entries)
		check(err)

		if fileCount > 0 {
			fmt.Println("Files found")
			fmt.Println(strconv.Itoa(fileCount))

			fileName := "chronicle-" + dateString + ".md"
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
}
