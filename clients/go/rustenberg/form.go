package rustenberg

import (
	"fmt"
	"mime/multipart"
	"reflect"
	"strings"

	"golang.org/x/exp/slices"
)

func writeToMultipartForm(formWriter *multipart.Writer, data interface{}) error {
	t := reflect.TypeOf(data)
	v := reflect.ValueOf(data)

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
