use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solrstice::{DefType, DismaxQuery, EdismaxQuery, LuceneQuery, QueryOperator};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[pyclass(name = "QueryOperator")]
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

#[pyclass(name = "DefType", module = "solrstice", subclass)]
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

#[pyclass(name = "LuceneQuery", extends=DefTypeWrapper, module = "solrstice", subclass)]
#[derive(Clone, Serialize, Deserialize)]
pub struct LuceneQueryWrapper {}

#[pymethods]
impl LuceneQueryWrapper {
    #[new]
    pub fn new(
        q_op: Option<QueryOperatorWrapper>,
        df: Option<String>,
        sow: Option<bool>,
    ) -> (Self, DefTypeWrapper) {
        let mut lucene = LuceneQuery::new();
        lucene.q_op = q_op.map(|q| q.into());
        lucene.df = df;
        lucene.sow = sow;
        (Self {}, DefTypeWrapper(DefType::Lucene(lucene)))
    }
}

#[pyclass(name = "DismaxQuery", extends=DefTypeWrapper, module = "solrstice", subclass)]
#[derive(Clone, Serialize, Deserialize)]
pub struct DismaxQueryWrapper {}

#[pymethods]
impl DismaxQueryWrapper {
    #[new]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        q_alt: Option<String>,
        qf: Option<String>,
        mm: Option<String>,
        pf: Option<String>,
        ps: Option<String>,
        qs: Option<String>,
        tie: Option<String>,
        bq: Option<Vec<String>>,
        bf: Option<Vec<String>>,
    ) -> (Self, DefTypeWrapper) {
        let mut dismax = DismaxQuery::new();
        dismax.q_alt = q_alt;
        dismax.qf = qf;
        dismax.mm = mm;
        dismax.pf = pf;
        dismax.ps = ps;
        dismax.qs = qs;
        dismax.tie = tie;
        dismax.bq = bq;
        dismax.bf = bf;
        (Self {}, DefTypeWrapper(DefType::Dismax(dismax)))
    }
}

#[pyclass(name = "EdismaxQuery", extends=DefTypeWrapper, module = "solrstice", subclass)]
#[derive(Clone, Serialize, Deserialize)]
pub struct EdismaxQueryWrapper {}

#[pymethods]
impl EdismaxQueryWrapper {
    #[new]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        q_alt: Option<String>,
        qf: Option<String>,
        mm: Option<String>,
        mm_auto_relax: Option<bool>,
        pf: Option<String>,
        pf2: Option<String>,
        pf3: Option<String>,
        ps: Option<String>,
        ps2: Option<String>,
        ps3: Option<String>,
        qs: Option<String>,
        tie: Option<String>,
        bq: Option<Vec<String>>,
        bf: Option<Vec<String>>,
        sow: Option<bool>,
        boost: Option<Vec<String>>,
        lowercase_operators: Option<bool>,
        stopwords: Option<bool>,
        uf: Option<String>,
    ) -> (Self, DefTypeWrapper) {
        let mut edismax = EdismaxQuery::new();
        edismax.q_alt = q_alt;
        edismax.qf = qf;
        edismax.mm = mm;
        edismax.mm_auto_relax = mm_auto_relax;
        edismax.pf = pf;
        edismax.pf2 = pf2;
        edismax.pf3 = pf3;
        edismax.ps = ps;
        edismax.ps2 = ps2;
        edismax.ps3 = ps3;
        edismax.qs = qs;
        edismax.tie = tie;
        edismax.bq = bq;
        edismax.bf = bf;
        edismax.sow = sow;
        edismax.boost = boost;
        edismax.lowercase_operators = lowercase_operators;
        edismax.stopwords = stopwords;
        edismax.uf = uf;
        (Self {}, DefTypeWrapper(DefType::Edismax(edismax)))
    }
}
