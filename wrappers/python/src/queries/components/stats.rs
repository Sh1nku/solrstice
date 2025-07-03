use pyo3::{pyclass, pymethods};
use solrstice::StatsComponent;

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "StatsComponent", module = "solrstice", subclass)]
pub struct StatsComponentWrapper(StatsComponent);

#[pymethods]
impl StatsComponentWrapper {
    #[new]
    pub fn new(fields: Option<Vec<String>>) -> Self {
        let mut component = StatsComponent::new();
        if let Some(fields) = fields {
            component = component.fields(fields);
        }
        StatsComponentWrapper(component)
    }
}

impl From<StatsComponentWrapper> for StatsComponent {
    fn from(wrapper: StatsComponentWrapper) -> Self {
        wrapper.0
    }
}

impl From<&StatsComponentWrapper> for StatsComponent {
    fn from(wrapper: &StatsComponentWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<StatsComponent> for StatsComponentWrapper {
    fn from(component: StatsComponent) -> Self {
        StatsComponentWrapper(component)
    }
}

impl From<&StatsComponent> for StatsComponentWrapper {
    fn from(component: &StatsComponent) -> Self {
        StatsComponentWrapper(component.clone())
    }
}
