package rustenberg

import (
	"fmt"
	"io"
	"net/http"
)

func transformHttpResponseError(response *http.Response) error {
	if response.StatusCode < 400 {
		return nil
	}

	body, err := io.ReadAll(response.Body)
	if err != nil {
		return err
	}

	return fmt.Errorf("http error %v: %v", response.StatusCode, string(body))
}
