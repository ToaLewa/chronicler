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

func getMakeYearLog(curr timepieces.Current, chData ChronoData) YearLog {
	yearLog, exists := chData[curr.Year]

	if !exists {
		yearLog = YearLog{}
	}

	chData[curr.Year] = yearLog
	return yearLog
}

func getMakeMonthLog(curr timepieces.Current, chData ChronoData) MonthLog {
	yearLog := getMakeYearLog(curr, chData)
	monthLog, exists := yearLog[curr.Month]

	if !exists {
		monthLog = MonthLog{}
	}

	chData[curr.Year][curr.Month] = monthLog
	return monthLog
}

func getMakeDayLog(curr timepieces.Current, chData ChronoData) DayLog {
	monthLog := getMakeMonthLog(curr, chData)

	dayLog, exists := monthLog[curr.Day]

	if !exists {
		dayLog = DayLog{
			Date: curr.DateString,
		}
	}

	chData[curr.Year][curr.Month][curr.Day] = dayLog
	return dayLog
}

func ReadToday(chData ChronoData) {
	timePieces := timepieces.GetCurrent()
	dayLog := getMakeDayLog(timePieces, chData)

	fmt.Println(dayLog.Date)

	for i := 0; i < len(dayLog.Entries); i++ {
		entry := dayLog.Entries[i]
		fmt.Printf("-%s %s\n", entry.Time, entry.Text)
	}
}

func AppendLogEntry(chData ChronoData, userText string) {
	timePieces := timepieces.GetCurrent()

	dayLog := getMakeDayLog(timePieces, chData)
	dayLog.Entries = append(dayLog.Entries, Entry{
		Time: timePieces.TimeString,
		Text: userText,
	})

	chData[timePieces.Year][timePieces.Month][timePieces.Day] = dayLog
}
