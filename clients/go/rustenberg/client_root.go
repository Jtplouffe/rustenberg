package rustenberg

import (
	"encoding/json"
	"net/http"
)

type ServiceInfo struct {
	Version string `json:"version"`
}

func (client *Client) GetServiceInfo() (*ServiceInfo, error) {
	request, err := client.newRequest("GET", "/")
	if err != nil {
		return nil, err
	}

	response, err := http.DefaultClient.Do(request)
	if err != nil {
		return nil, err
	}

	defer func() {
		if err := response.Body.Close(); err != nil {
			client.options.handleDeferedError(err)
		}
	}()

	if err := transformHttpResponseError(response); err != nil {
		return nil, err
	}

	var serviceInfo ServiceInfo
	if err := json.NewDecoder(response.Body).Decode(&serviceInfo); err != nil {
		return nil, err
	}

	return &serviceInfo, nil
}
