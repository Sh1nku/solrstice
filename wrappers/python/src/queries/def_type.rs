use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solrstice::queries::def_type::{DefType, Lucene, QueryOperator};

#[pymodule]
pub fn def_type(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QueryOperatorWrapper>()?;
    m.add_class::<DefTypeWrapper>()?;
    m.add_class::<DefTypeLucene>()?;
    Ok(())
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[pyclass(name = "QueryOperator", module = "solrstice.def_type")]
pub enum QueryOperatorWrapper {
    AND,
    OR,
}

impl From<QueryOperatorWrapper> for QueryOperator {
    fn from(value: QueryOperatorWrapper) -> Self {
        match value {
            QueryOperatorWrapper::AND => QueryOperator::AND,
            QueryOperatorWrapper::OR => QueryOperator::OR,
        }
    }
}

impl From<QueryOperator> for QueryOperatorWrapper {
    fn from(value: QueryOperator) -> Self {
        match value {
            QueryOperator::AND => QueryOperatorWrapper::AND,
            QueryOperator::OR => QueryOperatorWrapper::OR,
        }
    }
}

#[pyclass(name = "DefType", subclass, module = "solrstice.def_type")]
#[derive(Clone, Serialize, Deserialize)]
pub struct DefTypeWrapper(DefType);

impl From<DefTypeWrapper> for DefType {
    fn from(w: DefTypeWrapper) -> Self {
        w.0
    }
}

impl From<DefType> for DefTypeWrapper {
    fn from(d: DefType) -> Self {
        Self(d)
    }
}

impl DefTypeWrapper {
    pub fn get_def_type(&self) -> &DefType {
        &self.0
    }

    pub fn get_def_type_mut(&mut self) -> &mut DefType {
        &mut self.0
    }
}

#[pyclass(name = "DefTypeLucene", extends=DefTypeWrapper, module = "solrstice.def_type")]
#[derive(Clone, Serialize, Deserialize)]
pub struct DefTypeLucene {}

#[pymethods]
impl DefTypeLucene {
    #[new]
    pub fn new(
        q_op: Option<QueryOperatorWrapper>,
        df: Option<String>,
        sow: Option<bool>,
    ) -> (Self, DefTypeWrapper) {
        let mut lucene = Lucene::new();
        lucene.q_op = q_op.map(|q| q.into());
        lucene.df = df;
        lucene.sow = sow;
        (Self {}, DefTypeWrapper(DefType::Lucene(lucene)))
    }

    #[getter]
    pub fn get_q_op(self_: PyRef<'_, Self>) -> Option<QueryOperatorWrapper> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Lucene(l) => l.q_op.map(|q| q.into()),
            _ => None,
        }
    }

    #[setter]
    pub fn set_q_op(mut self_: PyRefMut<'_, Self>, q_op: Option<QueryOperatorWrapper>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Lucene(d) => d.q_op = q_op.map(|q| q.into()),
            _ => {
                let mut lucene = Lucene::new();
                lucene.q_op = q_op.map(|q| q.into());
                *def_type = DefType::Lucene(lucene);
            }
        }
    }

    #[getter]
    pub fn get_df(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Lucene(l) => l.df.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_df(mut self_: PyRefMut<'_, Self>, df: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Lucene(d) => d.df = df,
            _ => {
                let mut lucene = Lucene::new();
                lucene.df = df;
                *def_type = DefType::Lucene(lucene);
            }
        }
    }

    #[getter]
    pub fn get_sow(self_: PyRef<'_, Self>) -> Option<bool> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Lucene(l) => l.sow,
            _ => None,
        }
    }

    #[setter]
    pub fn set_sow(mut self_: PyRefMut<'_, Self>, sow: Option<bool>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Lucene(d) => d.sow = sow,
            _ => {
                let mut lucene = Lucene::new();
                lucene.sow = sow;
                *def_type = DefType::Lucene(lucene);
            }
        }
    }
}
