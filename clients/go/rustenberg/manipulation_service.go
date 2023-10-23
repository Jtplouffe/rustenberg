package rustenberg

import (
	"bytes"
	"io"
	"mime/multipart"
	"net/http"
)

type ManipulationService clientService

type MergeDto struct {
	Documents []File
}

func (service *ManipulationService) Merge(dto *MergeDto) ([]byte, error) {
	buffer := bytes.Buffer{}
	formWriter := multipart.NewWriter(&buffer)

	if err := writeFilesToMultipartForm(formWriter, "documents", dto.Documents); err != nil {
		return nil, err
	}

	if err := formWriter.Close(); err != nil {
		return nil, err
	}

	request, err := service.client.newRequest("POST", "/manipulation/merge")
	if err != nil {
		return nil, err
	}

	request.Body = io.NopCloser(&buffer)
	request.Header.Set("Content-Type", formWriter.FormDataContentType())

	response, err := http.DefaultClient.Do(request)
	if err != nil {
		return nil, err
	}

	defer func() {
		if err := response.Body.Close(); err != nil {
			service.client.options.handleDeferedError(err)
		}
	}()

	if err := transformHttpResponseError(response); err != nil {
		return nil, err
	}

	return io.ReadAll(response.Body)
}
