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

func getMakeYearLog(curr timepieces.Current, chFile ChronoData) YearLog {
	yearLog, exists := chFile[curr.Year]

	if !exists {
		yearLog = YearLog{}
	}

	chFile[curr.Year] = yearLog
	return yearLog
}

func getMakeMonthLog(curr timepieces.Current, chFile ChronoData) MonthLog {
	yearLog := getMakeYearLog(curr, chFile)
	monthLog, exists := yearLog[curr.Month]

	if !exists {
		monthLog = MonthLog{}
	}

	chFile[curr.Year][curr.Month] = monthLog
	return monthLog
}

func getMakeDayLog(curr timepieces.Current, chFile ChronoData) DayLog {
	monthLog := getMakeMonthLog(curr, chFile)

	dayLog, exists := monthLog[curr.Day]

	if !exists {
		dayLog = DayLog{
			Date: curr.DateString,
		}
	}

	chFile[curr.Year][curr.Month][curr.Day] = dayLog
	return dayLog
}

func ReadToday(chFile ChronoData) {
	timePieces := timepieces.GetCurrent()
	dayLog := getMakeDayLog(timePieces, chFile)

	for i := 0; i < len(dayLog.Entries); i++ {
		fmt.Println("Reading ...")
	}
}

func AppendLogEntry(chFile ChronoData, userText string) {
	timePieces := timepieces.GetCurrent()

	dayLog := getMakeDayLog(timePieces, chFile)
	dayLog.Entries = append(dayLog.Entries, Entry{
		Time: timePieces.TimeString,
		Text: userText,
	})

	chFile[timePieces.Year][timePieces.Month][timePieces.Day] = dayLog
}
