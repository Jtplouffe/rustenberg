package rustenberg

import "net/http"

type Client struct {
	options ClientOptions

	Conversion   *ConversionService
	Manipulation *ManipulationService
}

func NewClient(options ClientOptions) (*Client, error) {
	if err := options.validate(); err != nil {
		return nil, err
	}

	client := &Client{
		options: options,
	}

	clientService := clientService{
		client: client,
	}

	client.Conversion = (*ConversionService)(&clientService)
	client.Manipulation = (*ManipulationService)(&clientService)

	return client, nil
}

func (client *Client) url(path string) string {
	url := client.options.ServiceUrl
	if len(path) > 0 {
		if (url[len(url)-1] == '/') != (path[0] == '/') {
			url += path
		} else {
			url += path[1:]
		}
	}

	return url
}

func (client *Client) newRequest(method, path string) (*http.Request, error) {
	request, err := http.NewRequest(method, client.url(path), nil)
	if err != nil {
		return nil, err
	}

	request.Header.Set("User-Agent", "rustenberg-go")
	return request, nil
}
