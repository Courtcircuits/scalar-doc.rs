use actix_web::{HttpResponse, Responder};

use crate::{templatize, Documentation, Theme};

pub struct ActixDocumentation {
    documentation: Documentation,
}

impl ActixDocumentation {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            documentation: Documentation::new(title, content),
        }
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
