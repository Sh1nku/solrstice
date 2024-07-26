use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::PyDowncastError;
use pythonize::PythonizeError;
use solrstice::Error;

pub struct PyErrWrapper(PyErr);

impl From<Error> for PyErrWrapper {
    fn from(err: Error) -> PyErrWrapper {
        PyErrWrapper(PyRuntimeError::new_err(err.to_string()))
    }
}

impl From<PyErrWrapper> for PyErr {
    fn from(err: PyErrWrapper) -> PyErr {
        err.0
    }
}

impl From<PythonizeError> for PyErrWrapper {
    fn from(err: PythonizeError) -> Self {
        PyErrWrapper(PyRuntimeError::new_err(err.to_string()))
    }
}

impl From<PyDowncastError<'_>> for PyErrWrapper {
    fn from(err: PyDowncastError) -> Self {
        PyErrWrapper(PyRuntimeError::new_err(err.to_string()))
    }
}
