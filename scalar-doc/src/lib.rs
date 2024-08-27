use anyhow::Result;
use serde::Serialize;
use tera::{Context, Tera};

pub mod configuration;
#[cfg(feature = "actix")]
pub mod scalar_actix;

static TEMPLATE: &str = include_str!("templates/template.html");

#[derive(Clone)]
pub enum Theme {
    Alternate,
    Default,
    Moon,
    Purple,
    Solarized,
    BluePlanet,
    Saturn,
    Kepler,
    Mars,
    DeepSpace,
    None,
}

impl Serialize for Theme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Theme {
    fn as_str(&self) -> &str {
        match self {
            Theme::Alternate => "alternate",
            Theme::Default => "default",
            Theme::Moon => "moon",
            Theme::Purple => "purple",
            Theme::Solarized => "solarized",
            Theme::BluePlanet => "blue-planet",
            Theme::Saturn => "saturn",
            Theme::Kepler => "kepler",
            Theme::Mars => "mars",
            Theme::DeepSpace => "deep-space",
            Theme::None => "none",
        }
    }
}

pub struct Documentation {
    title: String,
    content: String,
    configuration: configuration::Configuration,
}

impl Documentation {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            configuration: configuration::Configuration::default(),
        }
    }

    pub fn theme(&mut self, theme: Theme) -> &mut Self {
        self.configuration.theme = Some(theme);
        self
    }

    pub fn build(&self) -> Result<String, anyhow::Error> {
        let configuration = serde_json::to_string(&self.configuration)?;
        templatize(self.content.clone(), self.title.clone(), configuration)
    }
}

pub fn templatize(
    content: String,
    title: String,
    configuration: String,
) -> Result<String, anyhow::Error> {
    let mut tera = Tera::default();
    tera.add_raw_template("template.html", TEMPLATE)?;

    let mut context = Context::new();
    context.insert("documentation", &content);
    context.insert("title", &title);
    context.insert("configuration", &configuration);

    let parsed_doc = tera.render("template.html", &context)?;

    Ok(parsed_doc)
}
