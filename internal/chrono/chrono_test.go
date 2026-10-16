package chrono

import "testing"

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
