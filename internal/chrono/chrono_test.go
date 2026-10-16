package chrono

import (
	"chronicler/internal/timepieces"
	"testing"
)

func TestCompareByTime(t *testing.T) {
	a := Entry{Time: "03:00", Text: "a"}
	b := Entry{Time: "04:00", Text: "b"}
	c := Entry{Time: "03:01", Text: "c"}
	d := Entry{Time: "04:00", Text: "d"}
	abCompare := compareEntriesByTime(a, b)
	baCompare := compareEntriesByTime(b, a)
	acCompare := compareEntriesByTime(a, c)
	bdCompare := compareEntriesByTime(b, d)

	if abCompare != -1 {
		t.Errorf("a should be smaller than b. Got: %d", abCompare)
	}

	if baCompare != 1 {
		t.Errorf("b should be larger than a. Got: %d", baCompare)
	}

	if acCompare != -1 {
		t.Errorf("a should be smaller than c. Got: %d", acCompare)
	}

	if bdCompare != 0 {
		t.Errorf("b should be equal to d. Got: %d", bdCompare)
	}
}

func TestDropTodayEntryPersistsDeletedEntry(t *testing.T) {
	chData := ChronoData{}
	AppendLogEntryWithTime(chData, "first", "09:00")
	AppendLogEntryWithTime(chData, "second", "10:00")

	DropTodayEntry(chData, 0)

	current := timepieces.GetCurrent()
	entries := chData[current.Year][current.Month][current.Day].Entries
	if len(entries) != 1 {
		t.Fatalf("expected one entry after deletion, got %d", len(entries))
	}
	if entries[0].Text != "second" {
		t.Errorf("expected remaining entry to be second, got %q", entries[0].Text)
	}
}
