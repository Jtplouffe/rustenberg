# Conversion

The conversion module is separated in two sections: web-based conversions, and non-web-based conversion.

Endpoints in this module only accept multipart body.

## Web-based

Web-based conversions are done using CDP, with a chromium instance.

In order to greatly improve performance, web-base conversions share the same chromium instance, with each conversion
being isolated into it's own browser context.

Web-based conversions share the same base options. No option is required.
Most of those options come directly from the CDP `printToPDF` function.
Details can be found [here](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF).

There is also a couple of options which are not part of the CDP `printToPDF` function:

| Name                | Type     | Description                                          |
|---------------------|----------|------------------------------------------------------|
| `minPageLoadWaitMs` | `number` | Minimum amount of time to wait for the page to load. |
| `maxPageLoadWaitMs` | `number` | Maximum amount of time to wait for the page to load. |

In addition to the `minPageLoadWaitMs` / `maxPageLoadWaitMs` options, multiple events will be awaited before generating
the pdf. Those events are: `networkIdle`, `domContentEvent`, `loadEvent`, and `loadingFinished`.

## Security

Although every request gets it's own unique browser context, these endpoints should only be called from a trustedclient
client, with trusted urls / files only.

Untrusted users should never be able to manipulate the content being loaded in the browser.
This microservice should therefore not be publicly accessible.

## Routes

### Url

The `POST /conversion/url` endpoint is used to convert a website page into a pdf document.

In addition to the base options, it takes a required `url` argument, which is the url of the web page that will be
converted to PDF.

Here is an example:
```sh
curl \
    --request POST "http://localhost:8000/conversion/url" \
    --form url="https://en.wikipedia.org/wiki/Main_Page"
```

### Html

The `POST /conversion/html` endpoint is used to convert html files into a pdf document.

In addition to the base options, it takes a required `files` argument, which is the files that will be converted to a
PDF document.
All files will be put into the same directory.
It is required that at least one of the files be named `index.html`.

Here is an example:

<sub>Filename: `index.html`</sub>
```html
<!doctype HTML>
<html>
    <body>
        <h1>Hello, World!</h1>
    </body>
</html>
```

```sh
curl \
    --request POST "http://localhost:8000/conversion/html" \
    --form files="@./index.html"
```
