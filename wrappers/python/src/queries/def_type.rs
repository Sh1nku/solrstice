use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solrstice::queries::def_type::{
    DefType, DismaxQueryBuilder, EdismaxQueryBuilder, LuceneQueryBuilder, QueryOperator,
};

#[pymodule]
pub fn def_type(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QueryOperatorWrapper>()?;
    m.add_class::<DefTypeQueryBuilder>()?;
    m.add_class::<LuceneQueryBuilderWrapper>()?;
    m.add_class::<DismaxQueryBuilderWrapper>()?;
    m.add_class::<EdismaxQueryBuilderWrapper>()?;
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

#[pyclass(name = "DefTypeQueryBuilder", subclass, module = "solrstice.def_type")]
#[derive(Clone, Serialize, Deserialize)]
pub struct DefTypeQueryBuilder(DefType);

impl From<DefTypeQueryBuilder> for DefType {
    fn from(w: DefTypeQueryBuilder) -> Self {
        w.0
    }
}

impl From<DefType> for DefTypeQueryBuilder {
    fn from(d: DefType) -> Self {
        Self(d)
    }
}

impl DefTypeQueryBuilder {
    pub fn get_def_type(&self) -> &DefType {
        &self.0
    }

    pub fn get_def_type_mut(&mut self) -> &mut DefType {
        &mut self.0
    }
}

#[pyclass(name = "LuceneQueryBuilder", extends=DefTypeQueryBuilder, module = "solrstice.def_type")]
#[derive(Clone, Serialize, Deserialize)]
pub struct LuceneQueryBuilderWrapper {}

#[pymethods]
impl LuceneQueryBuilderWrapper {
    #[new]
    pub fn new(
        q_op: Option<QueryOperatorWrapper>,
        df: Option<String>,
        sow: Option<bool>,
    ) -> (Self, DefTypeQueryBuilder) {
        let mut lucene = LuceneQueryBuilder::new();
        lucene.q_op = q_op.map(|q| q.into());
        lucene.df = df;
        lucene.sow = sow;
        (Self {}, DefTypeQueryBuilder(DefType::Lucene(lucene)))
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
                let mut lucene = LuceneQueryBuilder::new();
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
                let mut lucene = LuceneQueryBuilder::new();
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
                let mut lucene = LuceneQueryBuilder::new();
                lucene.sow = sow;
                *def_type = DefType::Lucene(lucene);
            }
        }
    }
}

#[pyclass(name = "DismaxQueryBuilder", extends=DefTypeQueryBuilder, module = "solrstice.def_type")]
#[derive(Clone, Serialize, Deserialize)]
pub struct DismaxQueryBuilderWrapper {}

#[pymethods]
impl DismaxQueryBuilderWrapper {
    #[new]
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
    ) -> (Self, DefTypeQueryBuilder) {
        let mut dismax = DismaxQueryBuilder::new();
        dismax.q_alt = q_alt;
        dismax.qf = qf;
        dismax.mm = mm;
        dismax.pf = pf;
        dismax.ps = ps;
        dismax.qs = qs;
        dismax.tie = tie;
        dismax.bq = bq;
        dismax.bf = bf;
        (Self {}, DefTypeQueryBuilder(DefType::Dismax(dismax)))
    }

    #[getter]
    pub fn get_q_alt(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Dismax(d) => d.q_alt.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_q_alt(mut self_: PyRefMut<'_, Self>, q_alt: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Dismax(d) => d.q_alt = q_alt,
            _ => {
                let mut dismax = DismaxQueryBuilder::new();
                dismax.q_alt = q_alt;
                *def_type = DefType::Dismax(dismax);
            }
        }
    }

    #[getter]
    pub fn get_qf(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Dismax(d) => d.qf.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_qf(mut self_: PyRefMut<'_, Self>, qf: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Dismax(d) => d.qf = qf,
            _ => {
                let mut dismax = DismaxQueryBuilder::new();
                dismax.qf = qf;
                *def_type = DefType::Dismax(dismax);
            }
        }
    }

    #[getter]
    pub fn get_mm(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Dismax(d) => d.mm.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_mm(mut self_: PyRefMut<'_, Self>, mm: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Dismax(d) => d.mm = mm,
            _ => {
                let mut dismax = DismaxQueryBuilder::new();
                dismax.mm = mm;
                *def_type = DefType::Dismax(dismax);
            }
        }
    }

    #[getter]
    pub fn get_pf(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Dismax(d) => d.pf.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_pf(mut self_: PyRefMut<'_, Self>, pf: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Dismax(d) => d.pf = pf,
            _ => {
                let mut dismax = DismaxQueryBuilder::new();
                dismax.pf = pf;
                *def_type = DefType::Dismax(dismax);
            }
        }
    }

    #[getter]
    pub fn get_ps(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Dismax(d) => d.ps.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_ps(mut self_: PyRefMut<'_, Self>, ps: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Dismax(d) => d.ps = ps,
            _ => {
                let mut dismax = DismaxQueryBuilder::new();
                dismax.ps = ps;
                *def_type = DefType::Dismax(dismax);
            }
        }
    }

    #[getter]
    pub fn get_qs(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Dismax(d) => d.qs.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_qs(mut self_: PyRefMut<'_, Self>, qs: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Dismax(d) => d.qs = qs,
            _ => {
                let mut dismax = DismaxQueryBuilder::new();
                dismax.qs = qs;
                *def_type = DefType::Dismax(dismax);
            }
        }
    }

    #[getter]
    pub fn get_tie(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Dismax(d) => d.tie.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_tie(mut self_: PyRefMut<'_, Self>, tie: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Dismax(d) => d.tie = tie,
            _ => {
                let mut dismax = DismaxQueryBuilder::new();
                dismax.tie = tie;
                *def_type = DefType::Dismax(dismax);
            }
        }
    }
}

#[pyclass(name = "EdismaxQueryBuilder", extends=DefTypeQueryBuilder, module = "solrstice.def_type")]
#[derive(Clone, Serialize, Deserialize)]
pub struct EdismaxQueryBuilderWrapper {}

#[pymethods]
impl EdismaxQueryBuilderWrapper {
    #[new]
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
    ) -> (Self, DefTypeQueryBuilder) {
        let mut edismax = EdismaxQueryBuilder::new();
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
        (Self {}, DefTypeQueryBuilder(DefType::Edismax(edismax)))
    }

    #[getter]
    pub fn get_q_alt(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.q_alt.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_q_alt(mut self_: PyRefMut<'_, Self>, q_alt: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.q_alt = q_alt,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.q_alt = q_alt;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_qf(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.qf.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_qf(mut self_: PyRefMut<'_, Self>, qf: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.qf = qf,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.qf = qf;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_mm(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.mm.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_mm(mut self_: PyRefMut<'_, Self>, mm: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.mm = mm,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.mm = mm;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_mm_auto_relax(self_: PyRef<'_, Self>) -> Option<bool> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.mm_auto_relax,
            _ => None,
        }
    }

    #[setter]
    pub fn set_mm_auto_relax(mut self_: PyRefMut<'_, Self>, mm_auto_relax: Option<bool>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.mm_auto_relax = mm_auto_relax,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.mm_auto_relax = mm_auto_relax;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_pf(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.pf.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_pf(mut self_: PyRefMut<'_, Self>, pf: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.pf = pf,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.pf = pf;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_pf2(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.pf2.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_pf2(mut self_: PyRefMut<'_, Self>, pf2: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.pf2 = pf2,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.pf2 = pf2;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_pf3(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.pf3.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_pf3(mut self_: PyRefMut<'_, Self>, pf3: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.pf3 = pf3,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.pf3 = pf3;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_ps(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.ps.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_ps(mut self_: PyRefMut<'_, Self>, ps: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.ps = ps,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.ps = ps;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_ps2(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.ps2.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_ps2(mut self_: PyRefMut<'_, Self>, ps2: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.ps2 = ps2,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.ps2 = ps2;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_ps3(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.ps3.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_ps3(mut self_: PyRefMut<'_, Self>, ps3: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.ps3 = ps3,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.ps3 = ps3;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_qs(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.qs.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_qs(mut self_: PyRefMut<'_, Self>, qs: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.qs = qs,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.qs = qs;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_tie(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.tie.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_tie(mut self_: PyRefMut<'_, Self>, tie: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.tie = tie,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.tie = tie;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_bq(self_: PyRef<'_, Self>) -> Option<Vec<String>> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.bq.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_bq(mut self_: PyRefMut<'_, Self>, bq: Option<Vec<String>>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.bq = bq,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.bq = bq;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_bf(self_: PyRef<'_, Self>) -> Option<Vec<String>> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.bf.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_bf(mut self_: PyRefMut<'_, Self>, bf: Option<Vec<String>>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.bf = bf,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.bf = bf;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_sow(self_: PyRef<'_, Self>) -> Option<bool> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.sow,
            _ => None,
        }
    }

    #[setter]
    pub fn set_sow(mut self_: PyRefMut<'_, Self>, sow: Option<bool>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.sow = sow,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.sow = sow;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_boost(self_: PyRef<'_, Self>) -> Option<Vec<String>> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.boost.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_boost(mut self_: PyRefMut<'_, Self>, boost: Option<Vec<String>>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.boost = boost,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.boost = boost;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_lowercase_operators(self_: PyRef<'_, Self>) -> Option<bool> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.lowercase_operators,
            _ => None,
        }
    }

    #[setter]
    pub fn set_lowercase_operators(
        mut self_: PyRefMut<'_, Self>,
        lowercase_operators: Option<bool>,
    ) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.lowercase_operators = lowercase_operators,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.lowercase_operators = lowercase_operators;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_stopwords(self_: PyRef<'_, Self>) -> Option<bool> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.stopwords,
            _ => None,
        }
    }

    #[setter]
    pub fn set_stopwords(mut self_: PyRefMut<'_, Self>, stopwords: Option<bool>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.stopwords = stopwords,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.stopwords = stopwords;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }

    #[getter]
    pub fn get_uf(self_: PyRef<'_, Self>) -> Option<String> {
        let super_ = self_.as_ref();
        let def_type = super_.get_def_type();
        match def_type {
            DefType::Edismax(d) => d.uf.clone(),
            _ => None,
        }
    }

    #[setter]
    pub fn set_uf(mut self_: PyRefMut<'_, Self>, uf: Option<String>) {
        let super_ = self_.as_mut();
        let def_type = super_.get_def_type_mut();
        match def_type {
            DefType::Edismax(d) => d.uf = uf,
            _ => {
                let mut edismax = EdismaxQueryBuilder::new();
                edismax.uf = uf;
                *def_type = DefType::Edismax(edismax);
            }
        }
    }
}
