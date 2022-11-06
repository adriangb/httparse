use std::str;

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyString};
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
    fn __repr__(&self, py: Python) -> PyResult<String> {
        let value = self.value.as_ref(py).as_bytes();
        Ok(format!(
            "Header(name=\"{}\", value=b\"{}\")",
            self.name,
            str::from_utf8(value)?
        ))
    }
    fn __str__(&self, py: Python) -> PyResult<String> {
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

#[pyclass(module = "httparse._httparse")]
#[derive(Clone, Debug)]
struct RequestParser {}

#[pymethods]
impl RequestParser {
    #[new]
    fn py_new() -> Self {
        RequestParser {}
    }
    fn parse(&mut self, buff: &PyBytes, py: Python) -> PyResult<Option<ParsedRequest>> {
        let mut headers = [httparse::EMPTY_HEADER; 256];
        let mut request = httparse::Request::new(&mut headers);
        let maybe_status = request.parse(buff.as_bytes());
        match maybe_status {
            Ok(status) => Ok({
                match status.is_complete() {
                    true => {
                        let header_list: Py<PyList> = PyList::new(
                            py,
                            request.headers.into_iter().map(|h| {
                                Py::new(
                                    py,
                                    Header {
                                        name: PyString::new(py, h.name).into(),
                                        value: PyBytes::new(py, h.value).into(),
                                    },
                                )
                                .unwrap()
                            }),
                        )
                        .into();
                        Some(ParsedRequest {
                            method: PyString::new(py, request.method.unwrap()).into(),
                            path: PyString::new(py, request.path.unwrap()).into(),
                            version: request.version.unwrap(),
                            headers: header_list,
                            body_start_offset: status.unwrap(),
                        })
                    }
                    false => None,
                }
            }),
            Err(parse_error) => match parse_error {
                httparse::Error::HeaderName => Err(InvalidHeaderName::new_err(())),
                httparse::Error::HeaderValue => Err(InvalidHeaderValue::new_err(())),
                httparse::Error::NewLine => Err(InvalidByteInNewLine::new_err(())),
                httparse::Error::Token => Err(InvalidToken::new_err(())),
                httparse::Error::TooManyHeaders => Err(TooManyHeaders::new_err(())),
                httparse::Error::Version => Err(InvalidHTTPVersion::new_err(())),
                _ => panic!(),
            },
        }
    }
}

#[pymodule]
fn _httparse(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Header>()?;
    m.add_class::<ParsedRequest>()?;
    m.add_class::<RequestParser>()?;
    m.add("InvalidChunkSize", _py.get_type::<InvalidChunkSize>())?;
    m.add("ParsingError", _py.get_type::<ParsingError>())?;
    m.add("InvalidHeaderName", _py.get_type::<InvalidHeaderName>())?;
    m.add("InvalidHeaderValue", _py.get_type::<InvalidHeaderValue>())?;
    m.add(
        "InvalidByteInNewLine",
        _py.get_type::<InvalidByteInNewLine>(),
    )?;
    m.add(
        "InvalidByteRangeInResponseStatus",
        _py.get_type::<InvalidByteRangeInResponseStatus>(),
    )?;
    m.add("InvalidToken", _py.get_type::<InvalidToken>())?;
    m.add("TooManyHeaders", _py.get_type::<TooManyHeaders>())?;
    m.add("InvalidHTTPVersion", _py.get_type::<InvalidHTTPVersion>())?;
    Ok(())
}
