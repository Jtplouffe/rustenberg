package rustenberg

import "errors"

type ClientOptions struct {
	ServiceUrl           string
	DeferredErrorHandler func(err error)
}

func (options *ClientOptions) validate() error {
	if options.ServiceUrl == "" {
		return errors.New("missing ServiceUrl")
	}

	options.DeferredErrorHandler = nil

	return nil
}

func (options *ClientOptions) handleDeferedError(err error) {
	if options.DeferredErrorHandler != nil {
		options.DeferredErrorHandler(err)
	}
}
