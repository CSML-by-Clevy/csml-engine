#[derive(Debug, Clone)]
pub struct Hold {
    pub index: usize,
    pub step_vars: serde_json::Value,
}
