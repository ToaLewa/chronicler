package chrono

import (
	"chronicler/internal/timepieces"
	"encoding/json"
	"errors"
	"fmt"
	"os"
)

// Use .chro file extension if .json is too constraining
type ChronoData map[int]YearLog

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

var ErrChronoFileNotFound = errors.New("chrono file not found")

func Load(fileName string) (ChronoData, error) {
	dat, readError := os.ReadFile(fileName)
	if errors.Is(readError, os.ErrNotExist) {
		return ChronoData{}, ErrChronoFileNotFound
	}
	if readError != nil {
		return ChronoData{}, fmt.Errorf("read %s: %w", fileName, readError)
	}

	var chronoFile ChronoData

	jsonError := json.Unmarshal(dat, &chronoFile)
	if jsonError != nil {
		return ChronoData{}, fmt.Errorf("parse %s: %w", fileName, jsonError)
	}

	return chronoFile, nil
}

func yearLogExists(curr timepieces.Current, chFile ChronoData) bool {
	_, ok := chFile[curr.Year]
	return ok
}

func monthLogExists(curr timepieces.Current, chFile ChronoData) bool {
	if !yearLogExists(curr, chFile) {
		return false
	}

	_, monthFound := chFile[curr.Year][curr.Month]

	return monthFound
}

func dayLogExists(curr timepieces.Current, chFile ChronoData) bool {
	if !monthLogExists(curr, chFile) {
		return false
	}

	_, dayFound := chFile[curr.Year][curr.Month][curr.Day]
	return dayFound
}

func ReadToday(chFile ChronoData) {
	fmt.Println("Reading ...")
}

func AppendLogEntry(chFile ChronoData, userText string) {
	timePieces := timepieces.GetCurrent()

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
