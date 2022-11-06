from __future__ import annotations

import sys
from typing import Sequence

if sys.version_info < (3, 8):
    from typing_extensions import Protocol
else:
    from typing import Protocol

class InvalidChunkSize(Exception):
    pass

class ParsingError(Exception):
    pass

class InvalidHeaderName(ParsingError):
    pass

class InvalidHeaderValue(ParsingError):
    pass

class InvalidByteInNewLine(ParsingError):
    pass

class InvalidByteRangeInResponseStatus(ParsingError):
    pass

class InvalidToken(ParsingError):
    pass

class TooManyHeaders(ParsingError):
    pass

class InvalidHTTPVersion(ParsingError):
    pass

class Header(Protocol):
    @property
    def name(self) -> str: ...
    @property
    def value(self) -> bytes: ...

class ParsedRequest:
    @property
    def method(self) -> str: ...
    @property
    def path(self) -> str: ...
    @property
    def version(self) -> int: ...
    @property
    def headers(self) -> Sequence[Header]: ...
    @property
    def body_start_offset(self) -> int: ...

class RequestParser:
    def parse(self, __buf: bytes) -> ParsedRequest | None: ...
