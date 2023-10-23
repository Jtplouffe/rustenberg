package rustenberg

import (
	"fmt"
	"mime/multipart"
	"reflect"
	"strings"

	"golang.org/x/exp/slices"
)

func writeDataToMultipartForm(formWriter *multipart.Writer, data interface{}) error {
	t := reflect.TypeOf(data)
	v := reflect.ValueOf(data)

	if t.Kind() == reflect.Ptr {
		t = t.Elem()
		v = v.Elem()
	}

	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		if !field.IsExported() {
			continue
		}

		value := v.Field(i)

		tag := field.Tag.Get("form")
		if tag == "" {
			continue
		}

		tagParts := strings.Split(tag, ",")
		fieldname := tagParts[0]
		if fieldname == "-" {
			continue
		}

		omitempty := len(tagParts) > 1 && slices.Contains(tagParts, "omitempty")

		if omitempty && value.IsZero() || (value.Kind() == reflect.Pointer && value.Elem().IsZero()) {
			continue
		}

		if err := formWriter.WriteField(fieldname, fmt.Sprintf("%v", value)); err != nil {
			return err
		}
	}

	return nil
}

func writeFilesToMultipartForm(formWriter *multipart.Writer, fieldname string, files []File) error {
	for _, file := range files {
		filename, err := file.name()
		if err != nil {
			return err
		}

		fileContent, err := file.content()
		if err != nil {
			return err
		}

		fileWriter, err := formWriter.CreateFormFile(fieldname, filename)
		if err != nil {
			return err
		}

		if _, err := fileWriter.Write(fileContent); err != nil {
			return err
		}
	}

	return nil
}
