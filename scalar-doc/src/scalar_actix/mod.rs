use actix_web::{HttpResponse, Responder};
use crate::{favicon::{Favicon, FaviconMimeType}, Documentation, Theme};

pub struct ActixDocumentation {
    documentation: Documentation,
}

impl ActixDocumentation {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            documentation: Documentation::new(title, content),
        }
    }

    pub fn favicon(&mut self, favicon: &str, mime: FaviconMimeType) -> &mut Self {
        self.documentation.favicon = Favicon::new(favicon.to_string(), mime);
        self
    }

    pub fn favicon_raw(&mut self, favicon: Favicon) -> &mut Self {
        self.documentation.favicon = favicon;
        self
    }

    pub fn theme(&mut self, theme: Theme) -> &mut Self {
        self.documentation.theme(theme);
        self
    }

    pub fn service(&self) -> impl Responder {
        let doc = self.documentation.build();
        HttpResponse::Ok().body(doc.unwrap())
    }
}
