use crate::models::error::PyErrWrapper;
use crate::models::response::SolrDocsResponseWrapper;
use crate::queries::components::grouping::{GroupFormattingWrapper, GroupingComponentWrapper};
use pyo3::prelude::*;
use pythonize::pythonize;
use solrstice::models::group::{SolrGroupFieldResult, SolrGroupResult};

#[pymodule]
pub fn group(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SolrGroupResultWrapper>()?;
    m.add_class::<SolrGroupFieldResultWrapper>()?;
    m.add_class::<GroupFormattingWrapper>()?;
    m.add_class::<GroupingComponentWrapper>()?;
    Ok(())
}

#[derive(Clone)]
#[pyclass(name = "SolrGroupResult", module = "solrstice.group")]
pub struct SolrGroupResultWrapper(SolrGroupResult);

#[derive(Clone)]
#[pyclass(name = "SolrGroupFieldResult", module = "solrstice.group")]
pub struct SolrGroupFieldResultWrapper {
    #[pyo3(get)]
    pub group_value: PyObject,
    #[pyo3(get)]
    pub doc_list: SolrDocsResponseWrapper,
}

#[pymethods]
impl SolrGroupResultWrapper {
    #[getter]
    pub fn get_matches(&self) -> usize {
        self.0.matches
    }

    #[getter]
    pub fn get_n_groups(&self) -> Option<usize> {
        self.0.n_groups
    }

    pub fn get_field_result(&self) -> PyResult<Option<Vec<SolrGroupFieldResultWrapper>>> {
        let result = self.0.get_field_result();
        match result {
            Some(v) => {
                let result = v
                    .iter()
                    .map(|v| {
                        SolrGroupFieldResultWrapper::try_from(v.clone()).map_err(PyErrWrapper::from)
                    })
                    .collect::<Result<Vec<SolrGroupFieldResultWrapper>, PyErrWrapper>>();
                Ok(Some(result?.to_vec()))
            }
            None => Ok(None),
        }
    }

    pub fn get_query_result(&self) -> PyResult<Option<SolrDocsResponseWrapper>> {
        match self.0.get_query_result() {
            Some(v) => Ok(Some(SolrDocsResponseWrapper::try_from(v.to_owned())?)),
            None => Ok(None),
        }
    }

    pub fn get_simple_result(&self) -> PyResult<Option<SolrDocsResponseWrapper>> {
        match self.0.get_simple_result() {
            Some(v) => Ok(Some(SolrDocsResponseWrapper::try_from(v.to_owned())?)),
            None => Ok(None),
        }
    }
}

impl From<SolrGroupResult> for SolrGroupResultWrapper {
    fn from(value: SolrGroupResult) -> Self {
        SolrGroupResultWrapper(value)
    }
}

impl TryFrom<SolrGroupFieldResult> for SolrGroupFieldResultWrapper {
    type Error = PyErrWrapper;

    fn try_from(value: SolrGroupFieldResult) -> Result<Self, Self::Error> {
        Python::with_gil(|py| -> Result<Self, Self::Error> {
            let group_value = pythonize(py, &value.group_value).map_err(PyErrWrapper::from)?;
            Ok(SolrGroupFieldResultWrapper {
                group_value,
                doc_list: SolrDocsResponseWrapper::try_from(value.doc_list)?,
            })
        })
    }
}
