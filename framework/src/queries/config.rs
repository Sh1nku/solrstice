use crate::error::Error;
use crate::models::context::SolrServerContext;
use crate::queries::request_builder::SolrRequestBuilder;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;
use tempfile::tempfile;
use walkdir::{DirEntry, WalkDir};
use zip::write::SimpleFileOptions;

// https://github.com/zip-rs/zip/blob/e32db515a2a4c7d04b0bf5851912a399a4cbff68/examples/write_dir.rs
fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &Path,
    writer: T,
    method: zip::CompressionMethod,
) -> Result<(), Error>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = SimpleFileOptions::default().compression_method(method);

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
pub async fn upload_config<C: AsRef<SolrServerContext>, S: AsRef<str>, P: AsRef<Path>>(
    context: C,
    name: S,
    path: P,
) -> Result<(), Error> {
    let query_params = [("action", "UPLOAD"), ("name", name.as_ref())];
    let mut outfile = tempfile()?;
    path.as_ref().try_exists()?;
    if path.as_ref().is_dir() {
        let walkdir = WalkDir::new(path.as_ref());
        let it = walkdir.into_iter();
        zip_dir(
            &mut it.filter_map(|e| e.ok()),
            path.as_ref(),
            &outfile,
            zip::CompressionMethod::Stored,
        )?;
        outfile.rewind()?;
    } else {
        outfile = File::open(path)?;
    }
    let mut vec = Vec::new();
    outfile.read_to_end(&mut vec)?;

    let _ = SolrRequestBuilder::new(context.as_ref(), "/solr/admin/configs")
        .with_query_params(query_params.as_ref())
        .with_headers(vec![("Content-Type", "application/octet-stream")])
        .send_post_with_body(vec)
        .await?;
    Ok(())
}

pub async fn get_configs<C: AsRef<SolrServerContext>>(context: C) -> Result<Vec<String>, Error> {
    let query_params = [("action", "LIST"), ("wt", "json")];
    let json = SolrRequestBuilder::new(context.as_ref(), "/solr/admin/configs")
        .with_query_params(query_params.as_ref())
        .send_get()
        .await?;
    match json.config_sets {
        None => Err(Error::Unknown("Could not get configsets".to_string())),
        Some(config_sets) => Ok(config_sets),
    }
}

pub async fn config_exists<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<bool, Error> {
    let configs = get_configs(context).await?;
    Ok(configs.contains(&name.as_ref().to_string()))
}

pub async fn delete_config<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<(), Error> {
    let query_params = [("action", "DELETE"), ("name", name.as_ref())];
    SolrRequestBuilder::new(context.as_ref(), "/solr/admin/configs")
        .with_query_params(query_params.as_ref())
        .send_get()
        .await?;
    Ok(())
}

#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;

#[cfg(feature = "blocking")]
pub fn upload_config_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>, P: AsRef<Path>>(
    context: C,
    name: S,
    path: P,
) -> Result<(), Error> {
    RUNTIME
        .handle()
        .block_on(upload_config(context, name, path))
}

#[cfg(feature = "blocking")]
pub fn get_configs_blocking<C: AsRef<SolrServerContext>>(context: C) -> Result<Vec<String>, Error> {
    RUNTIME.handle().block_on(get_configs(context))
}

#[cfg(feature = "blocking")]
pub fn config_exists_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<bool, Error> {
    RUNTIME.handle().block_on(config_exists(context, name))
}

#[cfg(feature = "blocking")]
pub fn delete_config_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<(), Error> {
    RUNTIME.handle().block_on(delete_config(context, name))
}
