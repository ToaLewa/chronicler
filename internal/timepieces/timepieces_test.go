package timepieces

import (
	"testing"
	"time"
)

func TestGetTimepieceConversion(t *testing.T) {
	specificDate := time.Date(2025, time.October, 16, 12, 0, 0, 0, time.UTC)
	timepiece := Get(specificDate)

	dateString := timepiece.DateString
	expected := "2025-10-16"

	if dateString != expected {
		t.Errorf("%s is not in the expected format", dateString)
	}
}
