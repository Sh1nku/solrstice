use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct StatsComponent {
    stats: bool,
    #[serde(rename = "stats.field")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    fields: Vec<String>,
}

impl StatsComponent {
    pub fn new() -> Self {
        StatsComponent {
            stats: true,
            fields: Vec::new(),
        }
    }

    pub fn fields<S: Into<String>, I: IntoIterator<Item = S>>(mut self, fields: I) -> Self {
        self.fields = fields.into_iter().map(|x| x.into()).collect();
        self
    }
}
