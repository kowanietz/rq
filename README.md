# $${\color{red}r}ust \space re{\color{red}q}uests$$

# Installation

`cargo install --path .`


# Usage (`rq --help`)

```
Usage: rq [OPTIONS] <URL>

Arguments:
  <URL>  URL to request

Options:
  -g, --get                GET request
  -p, --post               POST request
  -u, --put                PUT request
  -d, --delete             DELETE request
  -x, --patch              PATCH request
  -b, --body <BODY>        Request body (JSON string or key=value pairs)
  -v, --verbose <VERBOSE>  Verbose mode [default: 0]
  -h, --help               Print help
  -V, --version            Print version
```

# Examples

## GET request

```bash
rq -g postman-echo.com/get
```
rq <url> defaults to GET:
```
rq postman-echo.com/get
```

## POST request

```bash
rq -p postman-echo.com/post -b '{"message":"rq"}'
```
