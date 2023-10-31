package rustenberg

import (
	"bytes"
	"io"
	"net/http"
	"testing"
)

func TestTransformHttpResponseError(t *testing.T) {
	response200 := &http.Response{StatusCode: 200}

	err := transformHttpResponseError(response200)
	if err != nil {
		t.Fatalf("wanted '%v', got '%v'", nil, err)
	}

	response400 := &http.Response{
		StatusCode: 400,
		Body:       io.NopCloser(bytes.NewBuffer([]byte("{}"))),
	}

	err = transformHttpResponseError(response400)
	if err == nil || err.Error() != "http error 400: {}" {
		t.Fatalf("wanted an error, got '%v'", err)
	}

	response422 := &http.Response{
		StatusCode: 422,
		Body:       io.NopCloser(bytes.NewBuffer([]byte("{}"))),
	}

	err = transformHttpResponseError(response422)
	if err == nil || err.Error() != "http error 422: {}" {
		t.Fatalf("wanted an error, got '%v'", err)
	}

	response500 := &http.Response{
		StatusCode: 500,
		Body:       io.NopCloser(bytes.NewBuffer([]byte("{}"))),
	}

	err = transformHttpResponseError(response500)
	if err == nil || err.Error() != "http error 500: {}" {
		t.Fatalf("wanted an error, got '%v'", err)
	}

	response503 := &http.Response{
		StatusCode: 503,
		Body:       io.NopCloser(bytes.NewBuffer([]byte("{}"))),
	}

	err = transformHttpResponseError(response503)
	if err == nil || err.Error() != "http error 503: {}" {
		t.Fatalf("wanted an error, got '%v'", err)
	}
}
