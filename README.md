# rustenberg

[![Server checks](https://github.com/Jtplouffe/rustenberg/actions/workflows/server.yml/badge.svg)](https://github.com/Jtplouffe/rustenberg/actions/workflows/server.yml)

Inspired by [gotenberg](https://github.com/gotenberg/gotenberg), rustenberg is a microservice that is used to convert
and manipulate pdf documents.

## Why rustenberg?

The main reason why rustenberg was created is performance, especially for web-based pdf convertions (url, html).
Compared to gonetberg, the time to generate a single pdf document is approximately the same.

However, the real difference is when multiple documents are been generated at the same time.
Gotenberg instanciates a new chromium instance for each request, which drastically incease cpu and memory usage.
Rustenberg, on the other hand, reuses the same instance, with each request having it's own seperate browser context.

## Features

Detailed documentation about the features of rustenberg are available [here](./server/docs/README.md).

The short version is that rustenberg supports converting web pages (either via sending the html files or using an url)
to pdf documents. It also supports merging pdf documents togheter.

More features will be implemented in the future.

## Clients

Multiple clients will be implemented over time. They will be located in the [clients directory](./clients).

## License

This project is currently licensed under AGPL-3. This will likely change once the project is ready for general use.
