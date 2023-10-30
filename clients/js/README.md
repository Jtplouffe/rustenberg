# rustenberg-js

Javascript / Typescript client for rustenberg.

## Sample usage

```ts
const client = new Client({
    serviceUrl: "http://127.0.0.1:8000",
});

const pdfs = await Promise.all(
    ["https://google.com", "https://youtube.com", "https://github.com"].map((url) =>
        client.conversions.convertUrl({ url }),
    ),
);

const mergedPdf = await client.manipulations.merge({
    documents: pdfs.map((pdf, index) => ({
        buffer: pdf,
        filename: `${index}_file.pdf`,
    })),
});
```
