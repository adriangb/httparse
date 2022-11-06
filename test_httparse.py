from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from typing import Dict, Set

import pytest

from httparse import RequestParser, ParsedRequest


Headers = Dict[str, Set[bytes]]


@dataclass(frozen=True)
class ParsedResultWrapper:
    method: str
    path: str
    version: int
    headers: Headers
    body_start_offset: int

    @classmethod
    def from_res(cls, res: ParsedRequest) -> ParsedResultWrapper:
        headers: Dict[str, Set[bytes]] = defaultdict(set)
        for header in res.headers:
            for value in header.value.split(b","):
                headers[header.name].add(value)
        return cls(
            method=res.method,
            path=res.path,
            version=res.version,
            headers=headers,
            body_start_offset=res.body_start_offset,
        )


@pytest.mark.parametrize(
    "buff,expected",
    [
        (
            b"GET /index.html HTTP/1.1\r\nHost: example.domain\r\n\r\n",
            ParsedResultWrapper(method="GET", path="/index.html", version=1, headers={"Host": {b"example.domain"}}, body_start_offset=50),
        ),
        (
            b"PUT / HTTP/1.1\r\nX-Foo: foo1,foo2\r\nX-Bar: bar\r\nX-Foo: foo3\r\n\r\n",
            ParsedResultWrapper(method="PUT", path="/", version=1, headers={"X-Bar": {b"bar"}, "X-Foo": {b"foo1", b"foo2", b"foo3"}}, body_start_offset=61),
        ),
    ],
)
def test_parse_complete(
    buff: bytes,
    expected: ParsedResultWrapper,
) -> None:
    parser = RequestParser()
    parsed = parser.parse(buff)
    assert parsed is not None

    got = ParsedResultWrapper.from_res(parsed)

    assert got == expected


def test_parse_partial() -> None:
    parser = RequestParser()
    parsed = parser.parse(b"GET /index.html HTTP/1.1\r\nHost")

    assert parsed is None

    parsed = parser.parse(b"GET /index.html HTTP/1.1\r\nHost: example.domain\r\n\r\n")

    assert parsed is not None




if __name__ == "__main__":
    pytest.main(
        [
            __file__,
        ]
    )
