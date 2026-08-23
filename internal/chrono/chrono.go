package chrono

import (
	"chronicler/internal/timepieces"
	"encoding/json"
	"errors"
	"fmt"
	"os"
)

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

var ErrChronoFileNotFound = errors.New("chrono file not found")

func Load(fileName string) (ChronoFile, error) {
	dat, readError := os.ReadFile(fileName)
	if errors.Is(readError, os.ErrNotExist) {
		return ChronoFile{}, ErrChronoFileNotFound
	}
	if readError != nil {
		return ChronoFile{}, fmt.Errorf("read %s: %w", fileName, readError)
	}

	var chronoFile ChronoFile

	jsonError := json.Unmarshal(dat, &chronoFile)
	if jsonError != nil {
		return ChronoFile{}, fmt.Errorf("parse %s: %w", fileName, jsonError)
	}

	return chronoFile, nil
}

func AppendLogEntry(chFile ChronoFile, userText string) {
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
