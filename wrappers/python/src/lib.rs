pub mod clients;
pub mod hosts;
pub mod models;
pub mod queries;

use crate::clients::{AsyncSolrCloudClientWrapper, BlockingSolrCloudClientWrapper};
use crate::hosts::{
    SolrHostWrapper, SolrMultipleServerHostWrapper, SolrSingleServerHostWrapper,
    ZookeeperEnsembleHostConnectorWrapper, ZookeeperEnsembleHostWrapper,
};
use crate::models::auth::{SolrAuthWrapper, SolrBasicAuthWrapper};
use crate::models::context::{
    FastLoggingPolicyWrapper, LoggingPolicyWrapper, OffLoggingPolicyWrapper,
    PrettyLoggingPolicyWrapper, SolrServerContextWrapper,
};
use crate::models::facet_set::{SolrFacetSetResultWrapper, SolrPivotFacetResultWrapper};
use crate::models::group::{SolrGroupFieldResultWrapper, SolrGroupResultWrapper};
use crate::models::json_facet::SolrJsonFacetResponseWrapper;
use crate::models::response::{SolrDocsResponseWrapper, SolrResponseWrapper};
use crate::queries::alias::alias as alias_module;
use crate::queries::collection::collection as collection_module;
use crate::queries::components::facet_set::{
    FacetSetComponentWrapper, FieldFacetComponentWrapper, FieldFacetEntryWrapper,
    FieldFacetMethodWrapper, FieldFacetSortWrapper, PivotFacetComponentWrapper,
};
use crate::queries::components::grouping::{GroupFormattingWrapper, GroupingComponentWrapper};
use crate::queries::components::json_facet::{
    JsonFacetComponentWrapper, JsonFacetTypeWrapper, JsonQueryFacetWrapper, JsonStatFacetWrapper,
    JsonTermsFacetWrapper,
};
use crate::queries::config::config as config_module;
use crate::queries::def_type::{
    DefTypeWrapper, DismaxQueryWrapper, EdismaxQueryWrapper, LuceneQueryWrapper,
    QueryOperatorWrapper,
};
use crate::queries::index::{CommitTypeWrapper, DeleteQueryWrapper, UpdateQueryWrapper};
use crate::queries::select::SelectQueryWrapper;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

#[pymodule]
#[pyo3(name = "models")]
fn models_module(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SolrFacetSetResultWrapper>()?;
    m.add_class::<SolrPivotFacetResultWrapper>()?;
    m.add_class::<SolrGroupResultWrapper>()?;
    m.add_class::<SolrGroupFieldResultWrapper>()?;
    m.add_class::<SolrJsonFacetResponseWrapper>()?;
    m.add_class::<SolrResponseWrapper>()?;
    m.add_class::<SolrDocsResponseWrapper>()?;
    m.add_class::<SolrGroupResultWrapper>()?;
    m.add_class::<SolrGroupFieldResultWrapper>()?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn solrstice(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    let sys = PyModule::import_bound(_py, "sys")?;
    let sys_modules = sys.getattr("modules")?;
    let sys_modules: &Bound<'_, PyDict> = sys_modules.downcast()?;

    m.add_class::<SolrAuthWrapper>()?;
    m.add_class::<SolrBasicAuthWrapper>()?;
    m.add_class::<QueryOperatorWrapper>()?;
    m.add_class::<DefTypeWrapper>()?;
    m.add_class::<LuceneQueryWrapper>()?;

    m.add_class::<LuceneQueryWrapper>()?;
    m.add_class::<DismaxQueryWrapper>()?;
    m.add_class::<EdismaxQueryWrapper>()?;

    m.add_class::<FacetSetComponentWrapper>()?;
    m.add_class::<PivotFacetComponentWrapper>()?;
    m.add_class::<FieldFacetComponentWrapper>()?;
    m.add_class::<FieldFacetSortWrapper>()?;
    m.add_class::<FieldFacetMethodWrapper>()?;
    m.add_class::<FieldFacetEntryWrapper>()?;

    m.add_class::<JsonFacetComponentWrapper>()?;
    m.add_class::<JsonFacetTypeWrapper>()?;
    m.add_class::<JsonTermsFacetWrapper>()?;
    m.add_class::<JsonQueryFacetWrapper>()?;
    m.add_class::<JsonStatFacetWrapper>()?;

    m.add_class::<SolrHostWrapper>()?;
    m.add_class::<SolrSingleServerHostWrapper>()?;
    m.add_class::<SolrMultipleServerHostWrapper>()?;
    m.add_class::<ZookeeperEnsembleHostWrapper>()?;
    m.add_class::<ZookeeperEnsembleHostConnectorWrapper>()?;
    m.add_class::<LoggingPolicyWrapper>()?;
    m.add_class::<OffLoggingPolicyWrapper>()?;
    m.add_class::<FastLoggingPolicyWrapper>()?;
    m.add_class::<PrettyLoggingPolicyWrapper>()?;
    m.add_class::<SolrServerContextWrapper>()?;

    m.add_class::<GroupFormattingWrapper>()?;
    m.add_class::<GroupingComponentWrapper>()?;

    m.add_class::<SelectQueryWrapper>()?;
    m.add_class::<CommitTypeWrapper>()?;
    m.add_class::<UpdateQueryWrapper>()?;
    m.add_class::<DeleteQueryWrapper>()?;

    m.add_class::<AsyncSolrCloudClientWrapper>()?;
    m.add_class::<BlockingSolrCloudClientWrapper>()?;

    m.add_wrapped(wrap_pymodule!(config_module))?;
    sys_modules.set_item("solrstice.config", m.getattr("config")?)?;

    m.add_wrapped(wrap_pymodule!(collection_module))?;
    sys_modules.set_item("solrstice.collection", m.getattr("collection")?)?;

    m.add_wrapped(wrap_pymodule!(alias_module))?;
    sys_modules.set_item("solrstice.alias", m.getattr("alias")?)?;

    m.add_wrapped(wrap_pymodule!(models_module))?;
    sys_modules.set_item("solrstice.models", m.getattr("models")?)?;

    Ok(())
}
