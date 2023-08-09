use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Debug)]
/// This struct encapsulates the commit types for Solr's update and delete queries.
/// By default, a `Hard` commit is performed, equating to `commit=true`.
/// Conversely, a `Soft` commit corresponds to `softCommit=true`.
/// # Examples
/// ```
/// use solrstice::models::commit_type::CommitType;
/// use solrstice::queries::index::{DeleteQueryBuilder, UpdateQueryBuilder};
///
/// let update_query = UpdateQueryBuilder::new().commit_type(CommitType::Soft);
/// let delete_query = DeleteQueryBuilder::new().commit_type(CommitType::Soft);
pub enum CommitType {
    Hard,
    Soft,
}

impl Default for CommitType {
    fn default() -> Self {
        Self::Hard
    }
}
