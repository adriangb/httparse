# httparse

![CI](https://github.com/adriangb/httparse/actions/workflows/python.yaml/badge.svg)

Python wrapper for Rust's [httparse](https://github.com/seanmonstar/httparse).
See this project on [GitHub](https://github.com/adriangb/httparse).

## Example

```python
from httparse import RequestParser

parser = RequestParser()

buff = b"GET /index.html HTTP/1.1\r\nHost"
parsed = parser.parse(buff)
assert parsed is None

# a partial request, so we try again once we have more data
buff = b"GET /index.html HTTP/1.1\r\nHost: example.domain\r\n\r\n"
parsed = parser.parse(buff)
assert parsed is not None
assert parsed.method == "GET"
assert parsed.path == "/index.html"
assert parsed.version == 1
assert parsed.body_start_byte_offset == len(buff)
headers = [(h.name, h.value.encode()) for h in parsed.headers]
assert headers == [(b"Host", b"example.com")]
```
