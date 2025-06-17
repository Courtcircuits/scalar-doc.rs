use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum FaviconMimeType {
    /// .ico (image/x-icon)
    Ico,
    /// .png (image/png)
    Png,
    /// .svg (image/svg+xml)
    Svg,
    /// .gif (image/gif)
    Gif,
    /// .jpg (image/jpeg)
    Jpeg,
    /// .webp (image/webp)
    Webp
}

impl Display for FaviconMimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ico => write!(f, "image/x-icon"),
            Self::Png => write!(f, "image/png"),
            Self::Svg => write!(f, "image/svg+xml"),
            Self::Gif => write!(f, "image/gif"),
            Self::Jpeg => write!(f, "image/jpeg"),
            Self::Webp => write!(f, "image/webp")
        }
    }
}

/// Favicon for a documentation page
///
/// **Default** - ``favicon.ico`` (FaviconMimeType::Ico)
#[derive(Clone, Debug)]
pub struct Favicon {
    favicon: String,
    mime: FaviconMimeType
}

impl Favicon {
    pub fn new(favicon: String, mime: FaviconMimeType) -> Self {
        Self { favicon, mime }
    }

    pub fn favicon(&self) -> &String {
        &self.favicon
    }

    pub fn mime(&self) -> String {
        self.mime.to_string()
    }
}

impl Default for Favicon {
    fn default() -> Self {
        Self {
            favicon: String::from("favicon.ico"),
            mime: FaviconMimeType::Ico
        }
    }
}