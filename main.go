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
type ChronoFile map[int]YearLog

type YearLog map[int]MonthLog

type MonthLog map[int]DayLog

type DayLog struct {
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

	if hasArg() {
		userText := os.Args[1]

		chFile, err := loadChronoFile()
		if err != nil && err != ErrChronoFileNotFound {
			fmt.Fprintf(os.Stderr, "error: %v\n", err)
			os.Exit(1)
		}

		if chFile == nil {
			chFile = ChronoFile{}
		}

		appendLogEntry(chFile, userText)

		b, err := json.Marshal(chFile)
		if err != nil {
			fmt.Fprintf(os.Stderr, "error: encode %s: %v\n", ChronoFileName, err)
			os.Exit(1)
		}

		if err := os.WriteFile(ChronoFileName, b, 0666); err != nil {
			fmt.Fprintf(os.Stderr, "error: write %s: %v\n", ChronoFileName, err)
			os.Exit(1)
		}

		fmt.Println("Write file")

	} else {
		timePieces := getCurrentTimePieces()
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

func appendLogEntry(chFile ChronoFile, userText string) {
	timePieces := getCurrentTimePieces()

	yearLog, ok := chFile[timePieces.Year]
	if !ok {
		yearLog = YearLog{}
		chFile[timePieces.Year] = yearLog
	}

	monthLog, ok := yearLog[timePieces.Month]
	if !ok {
		monthLog = MonthLog{}
		yearLog[timePieces.Month] = monthLog
	}

	dayLog, ok := monthLog[timePieces.Day]
	if !ok {
		dayLog = DayLog{Date: timePieces.DateString}
	}

	dayLog.Entries = append(dayLog.Entries, Entry{
		Time: timePieces.TimeString,
		Text: userText,
	})
	monthLog[timePieces.Day] = dayLog
}
