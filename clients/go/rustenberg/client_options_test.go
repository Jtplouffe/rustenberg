package rustenberg

import (
	"errors"
	"testing"
)

func TestValidate(t *testing.T) {
	t.Run("invalid service url", func(t *testing.T) {
		options := ClientOptions{
			ServiceUrl: "",
		}

		err := options.validate()
		if !errors.Is(err, ErrMissingServiceUrl) {
			t.Fatalf("wanted '%v', got '%v'", ErrMissingServiceUrl, err)
		}

		options = ClientOptions{
			ServiceUrl: "      ",
		}

		err = options.validate()
		if !errors.Is(err, ErrMissingServiceUrl) {
			t.Fatalf("wanted '%v', got '%v'", ErrMissingServiceUrl, err)
		}
	})

	t.Run("valid service url", func(t *testing.T) {
		options := ClientOptions{
			ServiceUrl: "http://127.0.0.1:8000",
		}

		err := options.validate()
		if err != nil {
			t.Fatalf("wanted '%v', got '%v'", nil, err)
		}
	})
}

func TestHandleDeferredError(t *testing.T) {
	t.Run("without error handler", func(t *testing.T) {
		options := ClientOptions{}
		options.handleDeferredError(errors.New("this error should not be handled"))
	})

	t.Run("with error handler", func(t *testing.T) {
		var lastHandledError error
		handledErrorCount := 0

		options := ClientOptions{
			DeferredErrorHandler: func(err error) {
				lastHandledError = err
				handledErrorCount++
			},
		}

		err := errors.New("first error")
		options.handleDeferredError(err)
		if !errors.Is(lastHandledError, err) {
			t.Fatalf("wanted '%v', got '%v'", err, lastHandledError)
		}

		if handledErrorCount != 1 {
			t.Fatalf("wanted '%v', got '%v'", 1, handledErrorCount)
		}

		err = errors.New("second error")
		options.handleDeferredError(err)
		if !errors.Is(lastHandledError, err) {
			t.Fatalf("wanted '%v', got '%v'", err, lastHandledError)
		}

		if handledErrorCount != 2 {
			t.Fatalf("wanted '%v', got '%v'", 2, handledErrorCount)
		}

		err = errors.New("third error")
		options.handleDeferredError(err)
		if !errors.Is(lastHandledError, err) {
			t.Fatalf("wanted '%v', got '%v'", err, lastHandledError)
		}

		if handledErrorCount != 3 {
			t.Fatalf("wanted '%v', got '%v'", 3, handledErrorCount)
		}
	})

}
