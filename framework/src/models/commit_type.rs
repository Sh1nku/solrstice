use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Debug)]
/// This struct encapsulates the commit types for Solr's update and delete queries.
/// By default, a `Hard` commit is performed, equating to `commit=true`.
/// Conversely, a `Soft` commit corresponds to `softCommit=true`.
/// # Examples
/// ```
/// use solrstice::{CommitType, DeleteQuery, UpdateQuery};
///
/// let update_query = UpdateQuery::new().commit_type(CommitType::Soft);
/// let delete_query = DeleteQuery::new().commit_type(CommitType::Soft);
pub enum CommitType {
    Hard,
    Soft,
}

impl Default for CommitType {
    fn default() -> Self {
        Self::Hard
    }
}
