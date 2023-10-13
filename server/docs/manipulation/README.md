# Manipulation

The manipulation module is used to manipulate pdf documents.
For now, it only contains the `merge` endpoint.

## Merge

The `POST /manipulation/merge` endpoint is used to merge pdf documents.

It takes a single argument, called `documents`. This is the pdf documents that should be merged together.
Those documents will be merged in their filename's alphabetical order.

A minimum of 2 documents is required.

Here is an example:
```sh
curl \
    --request POST "http://localhost:8000/manipulation/merge" \
    --form documents="@./1_document.pdf" \
    --form documents="@./2_document.pdf"
```
