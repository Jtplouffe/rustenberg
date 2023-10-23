package rustenberg

import (
	"bytes"
	"mime/multipart"
	"testing"

	"golang.org/x/exp/slices"
)

func TestWriteToMultipartForm(t *testing.T) {
	data := &struct {
		One   string  `form:"one"`
		Two   bool    `form:"Two"`
		Three float64 `form:"FIELD_THREE"`
	}{
		One:   "field one",
		Two:   true,
		Three: 42.123456789,
	}

	b := bytes.Buffer{}
	formWriter := multipart.NewWriter(&b)
	if err := writeDataToMultipartForm(formWriter, data); err != nil {
		t.Fatal(err)
	}

	if err := formWriter.Close(); err != nil {
		t.Fatal(err)
	}

	form, err := multipart.NewReader(&b, formWriter.Boundary()).ReadForm(0)
	if err != nil {
		t.Fatal(err)
	}

	if len := len(form.Value); len != 3 {
		t.Fatalf("expected 3 values, got %v", len)
	}

	if len := len(form.File); len > 0 {
		t.Fatalf("expectod no file, got %v", len)
	}

	fieldOneValue, ok := form.Value["one"]
	if !ok {
		t.Fatal("missing field 'one'")
	}

	if slices.Compare(fieldOneValue, []string{"field one"}) != 0 {
		t.Fatalf("invalid field 'one' value: %v", fieldOneValue)
	}

	fieldTwoValue, ok := form.Value["Two"]
	if !ok {
		t.Fatal("missing field 'Two'")
	}

	if slices.Compare(fieldTwoValue, []string{"true"}) != 0 {
		t.Fatalf("invalid field 'Two' value: %v", fieldTwoValue)
	}

	fieldThreeValue, ok := form.Value["FIELD_THREE"]
	if !ok {
		t.Fatal("missing field 'FIELD_THREE'")
	}

	if slices.Compare(fieldThreeValue, []string{"42.123456789"}) != 0 {
		t.Fatalf("invalid field 'FIELD_THREE' value: %v", fieldThreeValue)
	}
}

func TestWriteToMultipartFormOmitEmpty(t *testing.T) {
	data := &struct {
		One   string  `form:"one"`
		Two   string  `form:"two,omitempty"`
		Three bool    `form:"three,omitempty"`
		Four  float64 `form:"four,omitempty"`
		Five  int     `form:"five,omitempty"`
	}{
		Five: 42,
	}

	b := bytes.Buffer{}
	formWriter := multipart.NewWriter(&b)
	if err := writeDataToMultipartForm(formWriter, data); err != nil {
		t.Fatal(err)
	}

	if err := formWriter.Close(); err != nil {
		t.Fatal(err)
	}

	form, err := multipart.NewReader(&b, formWriter.Boundary()).ReadForm(0)
	if err != nil {
		t.Fatal(err)
	}

	if len := len(form.Value); len != 2 {
		t.Fatalf("expected 2 value, got %v", len)
	}

	if len := len(form.File); len > 0 {
		t.Fatalf("expectod no file, got %v", len)
	}

	fieldOneValue, ok := form.Value["one"]
	if !ok {
		t.Fatal("missing field 'one'")
	}

	if slices.Compare(fieldOneValue, []string{""}) != 0 {
		t.Fatalf("invalid field 'one' value: %v", fieldOneValue)
	}

	fieldFiveValue, ok := form.Value["five"]
	if !ok {
		t.Fatal("missing field 'five'")
	}

	if slices.Compare(fieldFiveValue, []string{"42"}) != 0 {
		t.Fatalf("invalid field 'five' value: %v", fieldFiveValue)
	}
}
