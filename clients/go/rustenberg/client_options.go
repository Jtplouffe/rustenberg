package rustenberg

import (
	"errors"
	"strings"
)

var ErrMissingServiceUrl = errors.New("missing ServiceUrl")

type ClientOptions struct {
	ServiceUrl           string
	DeferredErrorHandler func(err error)
}

func (options *ClientOptions) validate() error {
	if strings.TrimSpace(options.ServiceUrl) == "" {
		return ErrMissingServiceUrl
	}

	return nil
}

func (options *ClientOptions) handleDeferredError(err error) {
	if options.DeferredErrorHandler != nil {
		options.DeferredErrorHandler(err)
	}
}
