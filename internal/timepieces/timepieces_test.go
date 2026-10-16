package timepieces

import (
	"testing"
	"time"
)

func TestGetTimepieceConversion(t *testing.T) {
	specificDate := time.Date(2025, time.October, 16, 12, 20, 0, 0, time.UTC)
	timepiece := Get(specificDate)

	dateString := timepiece.DateString
	expectedDateString := "2025-10-16"

	if dateString != expectedDateString {
		t.Errorf("%s is not in the expected date format", dateString)
	}

	timeString := timepiece.TimeString
	expectedTimeString := "7:20"

	if timeString != expectedTimeString {
		t.Errorf("%s is not in the expected time format", timeString)
	}
}
