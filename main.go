package main

import (
	"encoding/json"
	"errors"
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

var ErrChronoFileNotFound = errors.New("chrono file not found")

func loadChronoFile() (ChronoFile, error) {
	dat, readError := os.ReadFile(ChronoFileName)
	if errors.Is(readError, os.ErrNotExist) {
		return ChronoFile{}, ErrChronoFileNotFound
	}
	if readError != nil {
		return ChronoFile{}, fmt.Errorf("read %s: %w", ChronoFileName, readError)
	}

	var chronoFile ChronoFile

	jsonError := json.Unmarshal(dat, &chronoFile)
	if jsonError != nil {
		return ChronoFile{}, fmt.Errorf("parse %s: %w", ChronoFileName, jsonError)
	}

	return chronoFile, nil
}

type CurrentTimePieces struct {
	Year       int
	Month      int
	Day        int
	DateString string
	TimeString string
}

func getCurrentTimePieces() CurrentTimePieces {
	now := time.Now()
	dateString := now.Format("2006-01-02")
	hour, min, _ := time.Now().Local().Clock()
	timeString := fmt.Sprintf("%d:%02d", hour, min)
	year, month, day := now.Date()

	return CurrentTimePieces{
		Year:       year,
		Month:      int(month),
		Day:        day,
		DateString: dateString,
		TimeString: timeString,
	}
}

func main() {

	timePieces := getCurrentTimePieces()

	if hasArg() {
		userText := os.Args[1]

		_, err := loadChronoFile()
		if err != nil && err != ErrChronoFileNotFound {
			fmt.Fprintf(os.Stderr, "error: %v\n", err)
			os.Exit(1)
		}

		oFile := ChronoFile{
			Logs: []YearLog{
				{
					Year: timePieces.Year,
					Months: []MonthLog{{
						Month: timePieces.Month,
						Days: []DayLog{
							{
								Day:  timePieces.Day,
								Date: timePieces.DateString,
								Entries: []Entry{
									{
										Time: timePieces.TimeString,
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
		fmt.Println(timePieces.DateString)

		// Read the current directory (".")
		entries, err := os.ReadDir("/Users/kkulis/Documents/atg/chrono/")
		fileCount := len(entries)
		check(err)

		if fileCount > 0 {
			fmt.Println("Files found")
			fmt.Println(strconv.Itoa(fileCount))

			fileName := "chronicle-" + timePieces.DateString + ".md"
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
