use crate::models::context::SolrServerContext;
use crate::models::error::{try_solr_error, SolrError};
use crate::models::response::SolrResponse;
use crate::queries::helpers::basic_solr_request;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;
use tempfile::tempfile;
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;

// https://github.com/zip-rs/zip/blob/e32db515a2a4c7d04b0bf5851912a399a4cbff68/examples/write_dir.rs
fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &Path,
    writer: T,
    method: zip::CompressionMethod,
) -> Result<(), SolrError>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default().compression_method(method);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix)?;
        if path.is_file() {
            zip.start_file(name.to_str().unwrap(), options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_str().unwrap(), options)?;
        }
    }
    zip.finish()?;
    Ok(())
}
pub async fn upload_config(
    builder: &SolrServerContext,
    name: &str,
    path: &Path,
) -> Result<(), SolrError> {
    let query_params = [("action", "UPLOAD"), ("name", name)];
    let mut request = builder
        .client
        .post(format!(
            "{}/solr/admin/configs",
            builder.host.get_solr_node().await?
        ))
        .header("Content-Type", "application/octet-stream")
        .query(&query_params);
    if let Some(auth) = &builder.auth {
        request = auth.add_auth_to_request(request)
    }
    let mut outfile = tempfile()?;
    path.try_exists()?;
    if path.is_dir() {
        let walkdir = WalkDir::new(path);
        let it = walkdir.into_iter();
        zip_dir(
            &mut it.filter_map(|e| e.ok()),
            path,
            &outfile,
            zip::CompressionMethod::Stored,
        )?;
        outfile.rewind()?;
    } else {
        outfile = File::open(path)?;
    }
    let mut vec = Vec::new();
    outfile.read_to_end(&mut vec)?;
    request = request.body(vec);
    let json = request.send().await?.json::<SolrResponse>().await?;
    try_solr_error(&json)?;
    Ok(())
}

pub async fn get_configs(builder: &SolrServerContext) -> Result<Vec<String>, SolrError> {
    let query_params = [("action", "LIST"), ("wt", "json")];
    let json = basic_solr_request(builder, "/solr/admin/configs", query_params.as_ref()).await?;
    match json.config_sets {
        None => Err(SolrError::Unknown("Could not get configsets".to_string())),
        Some(config_sets) => Ok(config_sets),
    }
}

pub async fn config_exists(builder: &SolrServerContext, name: &str) -> Result<bool, SolrError> {
    let configs = get_configs(builder).await?;
    Ok(configs.contains(&name.to_string()))
}

pub async fn delete_config(builder: &SolrServerContext, name: &str) -> Result<(), SolrError> {
    let query_params = [("action", "DELETE"), ("name", name)];
    basic_solr_request(builder, "/solr/admin/configs", query_params.as_ref()).await?;
    Ok(())
}

#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;

#[cfg(feature = "blocking")]
pub fn upload_config_blocking(
    builder: &SolrServerContext,
    name: &str,
    path: &Path,
) -> Result<(), SolrError> {
    RUNTIME
        .handle()
        .block_on(upload_config(builder, name, path))
}

#[cfg(feature = "blocking")]
pub fn get_configs_blocking(builder: &SolrServerContext) -> Result<Vec<String>, SolrError> {
    RUNTIME.handle().block_on(get_configs(builder))
}

#[cfg(feature = "blocking")]
pub fn config_exists_blocking(builder: &SolrServerContext, name: &str) -> Result<bool, SolrError> {
    RUNTIME.handle().block_on(config_exists(builder, name))
}

#[cfg(feature = "blocking")]
pub fn delete_config_blocking(builder: &SolrServerContext, name: &str) -> Result<(), SolrError> {
    RUNTIME.handle().block_on(delete_config(builder, name))
}
