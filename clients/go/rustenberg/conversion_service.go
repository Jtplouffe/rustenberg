package rustenberg

import (
	"bytes"
	"io"
	"mime/multipart"
	"net/http"
)

type ConversionService clientService

type ConvertUrlDto struct {
	Url                 string  `form:"url"`
	Landscape           bool    `form:"landscape,omitempty"`
	DisplayHeaderFooter bool    `form:"displayHeaderFooter,omitempty"`
	PrintBackground     bool    `form:"printBackground,omitempty"`
	Scale               float64 `form:"scale,omitempty"`
	PaperWidth          float64 `form:"paperWidth,omitempty"`
	PaperHeight         float64 `form:"paperHeight,omitempty"`
	MarginTop           float64 `form:"marginTop,omitempty"`
	MarginBottom        float64 `form:"marginBottom,omitempty"`
	MarginLeft          float64 `form:"marginLeft,omitempty"`
	MarginRight         float64 `form:"marginRight,omitempty"`
	PageRange           string  `form:"pageRange,omitempty"`
	HeaderTemplate      string  `form:"headerTemplate,omitempty"`
	FooterTemplate      string  `form:"footerTemplate,omitempty"`
	PreferCssPageSize   bool    `form:"preferCssPageSize,omitempty"`
	MinPageLoadTimeMs   uint    `form:"minPageLoadTimeMs,omitempty"`
	MaxPageLoadTimeMs   uint    `form:"maxPageLoadTimeMs,omitempty"`
}

func (service *ConversionService) ConvertUrl(dto *ConvertUrlDto) ([]byte, error) {
	buffer := bytes.Buffer{}
	formWriter := multipart.NewWriter(&buffer)

	if err := writeDataToMultipartForm(formWriter, dto); err != nil {
		formWriter.Close()
		return nil, err
	}

	if err := formWriter.Close(); err != nil {
		return nil, err
	}

	request, err := service.client.newRequest("POST", "/conversion/url")
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

type ConvertHtmlDto struct {
	Files               []File
	Landscape           bool    `form:"landscape,omitempty"`
	DisplayHeaderFooter bool    `form:"displayHeaderFooter,omitempty"`
	PrintBackground     bool    `form:"printBackground,omitempty"`
	Scale               float64 `form:"scale,omitempty"`
	PaperWidth          float64 `form:"paperWidth,omitempty"`
	PaperHeight         float64 `form:"paperHeight,omitempty"`
	MarginTop           float64 `form:"marginTop,omitempty"`
	MarginBottom        float64 `form:"marginBottom,omitempty"`
	MarginLeft          float64 `form:"marginLeft,omitempty"`
	MarginRight         float64 `form:"marginRight,omitempty"`
	PageRange           string  `form:"pageRange,omitempty"`
	HeaderTemplate      string  `form:"headerTemplate,omitempty"`
	FooterTemplate      string  `form:"footerTemplate,omitempty"`
	PreferCssPageSize   bool    `form:"preferCssPageSize,omitempty"`
	MinPageLoadTimeMs   uint    `form:"minPageLoadTimeMs,omitempty"`
	MaxPageLoadTimeMs   uint    `form:"maxPageLoadTimeMs,omitempty"`
}

func (service *ConversionService) ConvertHtml(dto *ConvertHtmlDto) ([]byte, error) {
	buffer := bytes.Buffer{}
	formWriter := multipart.NewWriter(&buffer)

	if err := writeDataToMultipartForm(formWriter, dto); err != nil {
		formWriter.Close()
		return nil, err
	}

	if err := writeFilesToMultipartForm(formWriter, "files", dto.Files); err != nil {
		return nil, err
	}

	if err := formWriter.Close(); err != nil {
		return nil, err
	}

	request, err := service.client.newRequest("POST", "/conversion/html")
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
