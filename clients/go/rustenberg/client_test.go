package rustenberg

import "testing"

func TestUrl(t *testing.T) {
	data := []struct {
		serviceUrl string
		path       string
		want       string
	}{
		{serviceUrl: "http://127.0.0.1:8000", path: "", want: "http://127.0.0.1:8000"},
		{serviceUrl: "http://127.0.0.1:8000/", path: "", want: "http://127.0.0.1:8000"},
		{serviceUrl: "http://127.0.0.1:8000", path: "/", want: "http://127.0.0.1:8000"},
		{serviceUrl: "http://127.0.0.1:8000/", path: "/", want: "http://127.0.0.1:8000"},
		{serviceUrl: "http://127.0.0.1:8000", path: "test", want: "http://127.0.0.1:8000/test"},
		{serviceUrl: "http://127.0.0.1:8000/", path: "test", want: "http://127.0.0.1:8000/test"},
		{serviceUrl: "http://127.0.0.1:8000", path: "/test", want: "http://127.0.0.1:8000/test"},
		{serviceUrl: "http://127.0.0.1:8000", path: "test/", want: "http://127.0.0.1:8000/test"},
		{serviceUrl: "http://127.0.0.1:8000/", path: "/test", want: "http://127.0.0.1:8000/test"},
		{serviceUrl: "http://127.0.0.1:8000/", path: "test/", want: "http://127.0.0.1:8000/test"},
		{serviceUrl: "http://127.0.0.1:8000/", path: "/test/", want: "http://127.0.0.1:8000/test"},
		{serviceUrl: "http://127.0.0.1:8000/", path: "/test/path", want: "http://127.0.0.1:8000/test/path"},
		{serviceUrl: "http://127.0.0.1:8000/", path: "/test/path/", want: "http://127.0.0.1:8000/test/path"},
		{serviceUrl: "http://127.0.0.1:8000/", path: "test/path/", want: "http://127.0.0.1:8000/test/path"},
		{serviceUrl: "http://127.0.0.1:8000/", path: "test/path", want: "http://127.0.0.1:8000/test/path"},
	}

	for _, data := range data {
		client, err := NewClient(ClientOptions{ServiceUrl: data.serviceUrl})
		if err != nil {
			t.Fatal(err)
		}

		got := client.url(data.path)
		if got != data.want {
			t.Fatalf("wanted '%v', got '%v'", data.want, got)
		}
	}
}
