pub mod clients;
pub mod hosts;
pub mod models;
pub mod queries;

use crate::clients::clients as clients_module;
use crate::hosts::hosts as hosts_module;
use crate::models::auth::auth as auth_module;
use crate::models::facet_set::facet_set as facet_set_module;
use crate::models::group::group as group_module;
use crate::models::json_facet::json_facet as json_facet_module;
use crate::models::response::response as response_module;
use crate::queries::alias::alias as alias_module;
use crate::queries::collection::collection as collection_module;
use crate::queries::config::config as config_module;
use crate::queries::def_type::def_type as def_type_module;
use crate::queries::index::{CommitTypeWrapper, DeleteQueryWrapper, UpdateQueryWrapper};
use crate::queries::select::SelectQueryWrapper;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

#[pymodule]
#[pyo3(name = "queries")]
fn queries_module(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SelectQueryWrapper>()?;
    m.add_class::<UpdateQueryWrapper>()?;
    m.add_class::<DeleteQueryWrapper>()?;
    m.add_class::<CommitTypeWrapper>()?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn solrstice(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys = PyModule::import_bound(_py, "sys")?;
    let sys_modules = sys.getattr("modules")?;
    let sys_modules: &Bound<'_, PyDict> = sys_modules.downcast()?;

    m.add_wrapped(wrap_pymodule!(config_module))?;
    sys_modules.set_item("solrstice.config", m.getattr("config")?)?;

    m.add_wrapped(wrap_pymodule!(collection_module))?;
    sys_modules.set_item("solrstice.collection", m.getattr("collection")?)?;

    m.add_wrapped(wrap_pymodule!(alias_module))?;
    sys_modules.set_item("solrstice.alias", m.getattr("alias")?)?;

    m.add_wrapped(wrap_pymodule!(clients_module))?;
    sys_modules.set_item("solrstice.clients", m.getattr("clients")?)?;

    m.add_wrapped(wrap_pymodule!(hosts_module))?;
    sys_modules.set_item("solrstice.hosts", m.getattr("hosts")?)?;

    m.add_wrapped(wrap_pymodule!(auth_module))?;
    sys_modules.set_item("solrstice.auth", m.getattr("auth")?)?;

    m.add_wrapped(wrap_pymodule!(queries_module))?;
    sys_modules.set_item("solrstice.queries", m.getattr("queries")?)?;

    m.add_wrapped(wrap_pymodule!(response_module))?;
    sys_modules.set_item("solrstice.response", m.getattr("response")?)?;

    m.add_wrapped(wrap_pymodule!(group_module))?;
    sys_modules.set_item("solrstice.group", m.getattr("group")?)?;

    m.add_wrapped(wrap_pymodule!(def_type_module))?;
    sys_modules.set_item("solrstice.def_type", m.getattr("def_type")?)?;

    m.add_wrapped(wrap_pymodule!(facet_set_module))?;
    sys_modules.set_item("solrstice.facet_set", m.getattr("facet_set")?)?;

    m.add_wrapped(wrap_pymodule!(json_facet_module))?;
    sys_modules.set_item("solrstice.json_facet", m.getattr("json_facet")?)?;
    Ok(())
}
