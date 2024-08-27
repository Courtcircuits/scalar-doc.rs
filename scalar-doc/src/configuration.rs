use crate::Theme;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Configuration {
    pub theme: Option<Theme>,
}

#[derive(Serialize, Clone)]
pub struct Spec {
    content: Option<String>,
    url: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            theme: Some(Theme::Default),
        }
    }
}
