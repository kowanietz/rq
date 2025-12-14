# $${\color{red}r}ust \space re{\color{red}q}uests$$

# Installation

`cargo install --path .`


# Usage (`rq --help`)

```
Usage: rq [OPTIONS] [URL]
       rq <COMMAND>

Commands:
  get     Send a GET request
  post    Send a POST request
  put     Send a PUT request
  delete  Send a DELETE request
  patch   Send a PATCH request
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [URL]  URL to request (when not using subcommands)

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

Using subcommand syntax (recommended):
```bash
rq get postman-echo.com/get
```

Using flag syntax:
```bash
rq -g postman-echo.com/get
```

Or just use the URL (defaults to GET):
```bash
rq postman-echo.com/get
```

## POST request

Using subcommand syntax (recommended):
```bash
rq post postman-echo.com/post -b '{"message":"rq"}'
```

Using flag syntax:
```bash
rq -p postman-echo.com/post -b '{"message":"rq"}'
```

## Other HTTP methods

All methods support both syntaxes:
```bash
# Subcommand style
rq put postman-echo.com/put -b '{"data":"value"}'
rq delete postman-echo.com/delete
rq patch postman-echo.com/patch -b '{"field":"update"}'

# Flag style
rq -u postman-echo.com/put -b '{"data":"value"}'
rq -d postman-echo.com/delete
rq -x postman-echo.com/patch -b '{"field":"update"}'
```
