use std::str;

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes, PyList, PyString};
use pyo3::Python;

create_exception!(_httparse, InvalidChunkSize, PyException);
create_exception!(_httparse, ParsingError, PyException);
create_exception!(_httparse, InvalidHeaderName, ParsingError);
create_exception!(_httparse, InvalidHeaderValue, ParsingError);
create_exception!(_httparse, InvalidByteInNewLine, ParsingError);
create_exception!(_httparse, InvalidByteRangeInResponseStatus, ParsingError);
create_exception!(_httparse, InvalidToken, ParsingError);
create_exception!(_httparse, TooManyHeaders, ParsingError);
create_exception!(_httparse, InvalidHTTPVersion, ParsingError);
create_exception!(_httparse, InvalidStatus, ParsingError);

#[pyclass(module = "httparse._httparse")]
#[derive(Clone, Debug)]
struct Header {
    #[pyo3(get)]
    name: Py<PyString>,
    #[pyo3(get)]
    value: Py<PyBytes>,
}

#[pymethods]
impl Header {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let value = self.value.as_ref(py).as_bytes();
        Ok(format!(
            "Header(name=\"{}\", value=b\"{}\")",
            self.name,
            str::from_utf8(value)?
        ))
    }
    fn __str__(&self, py: Python<'_>) -> PyResult<String> {
        self.__repr__(py)
    }
}

#[pyclass(module = "httparse._httparse")]
#[derive(Clone, Debug)]
struct ParsedRequest {
    #[pyo3(get)]
    method: Py<PyString>,
    #[pyo3(get)]
    path: Py<PyString>,
    #[pyo3(get)]
    version: u8,
    #[pyo3(get)]
    body_start_offset: usize,
    #[pyo3(get)]
    headers: Py<PyList>,
}

macro_rules! intern_match {
    ($py:expr, $value: expr, $($interned:expr),+) => {
        match $value {
            $($interned => ::pyo3::intern!($py, $interned),)+
            name => ::pyo3::types::PyString::new($py, name),
        }
    };
}

#[pyclass(module = "httparse._httparse")]
#[derive(Clone, Debug)]
struct RequestParser {}

#[derive(FromPyObject)]
enum PyData<'a> {
    Bytes(&'a PyBytes),
    ByteArray(&'a PyByteArray),
}

#[pymethods]
impl RequestParser {
    #[new]
    fn py_new() -> Self {
        RequestParser {}
    }
    fn parse(&mut self, buff: PyData, py: Python<'_>) -> PyResult<Option<ParsedRequest>> {
        let mut empty_headers = [httparse::EMPTY_HEADER; 256];
        let mut request = httparse::Request::new(&mut empty_headers);
        let maybe_status = request.parse(match buff {
            PyData::Bytes(d) => d.as_bytes(),
            PyData::ByteArray(d) => unsafe { d.as_bytes() },
        });
        match maybe_status {
            Ok(httparse::Status::Complete(body_start_offset)) => {
                let headers: Py<PyList> = PyList::new(
                    py,
                    request.headers.iter_mut().map(|h| {
                        Py::new(
                            py,
                            Header {
                                name: {
                                    intern_match!(
                                        py,
                                        h.name,
                                        "Host",
                                        "Connection",
                                        "Cache-Control",
                                        "Accept",
                                        "User-Agent",
                                        "Accept-Encoding",
                                        "Accept-Language",
                                        "Accept-Charset",
                                        "Cookie"
                                    )
                                }
                                .into(),
                                value: PyBytes::new(py, h.value).into(),
                            },
                        )
                        .unwrap()
                    }),
                )
                .into();
                let method = intern_match!(
                    py,
                    request.method.unwrap(),
                    "GET",
                    "POST",
                    "PUT",
                    "PATCH",
                    "DELETE",
                    "HEAD",
                    "OPTIONS",
                    "TRACE",
                    "CONNECT"
                )
                .into();

                Ok(Some(ParsedRequest {
                    method,
                    path: PyString::new(py, request.path.unwrap()).into(),
                    version: request.version.unwrap(),
                    headers,
                    body_start_offset,
                }))
            }
            Ok(httparse::Status::Partial) => Ok(None),
            Err(parse_error) => match parse_error {
                httparse::Error::HeaderName => Err(InvalidHeaderName::new_err(())),
                httparse::Error::HeaderValue => Err(InvalidHeaderValue::new_err(())),
                httparse::Error::NewLine => Err(InvalidByteInNewLine::new_err(())),
                httparse::Error::Token => Err(InvalidToken::new_err(())),
                httparse::Error::TooManyHeaders => Err(TooManyHeaders::new_err(())),
                httparse::Error::Version => Err(InvalidHTTPVersion::new_err(())),
                httparse::Error::Status => Err(InvalidStatus::new_err(())),
            },
        }
    }
}

#[pymodule]
fn _httparse(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Header>()?;
    m.add_class::<ParsedRequest>()?;
    m.add_class::<RequestParser>()?;
    m.add("InvalidChunkSize", py.get_type::<InvalidChunkSize>())?;
    m.add("ParsingError", py.get_type::<ParsingError>())?;
    m.add("InvalidHeaderName", py.get_type::<InvalidHeaderName>())?;
    m.add("InvalidHeaderValue", py.get_type::<InvalidHeaderValue>())?;
    m.add(
        "InvalidByteInNewLine",
        py.get_type::<InvalidByteInNewLine>(),
    )?;
    m.add(
        "InvalidByteRangeInResponseStatus",
        py.get_type::<InvalidByteRangeInResponseStatus>(),
    )?;
    m.add("InvalidToken", py.get_type::<InvalidToken>())?;
    m.add("TooManyHeaders", py.get_type::<TooManyHeaders>())?;
    m.add("InvalidHTTPVersion", py.get_type::<InvalidHTTPVersion>())?;
    m.add("InvalidStatus", py.get_type::<InvalidStatus>())?;
    Ok(())
}
