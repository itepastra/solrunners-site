//! # Solrunners website
//! Website for the Solrunners project.
//! Written in Rust using the Rocket web framework.

#[macro_use] extern crate rocket;

use lazy_static::lazy_static;
use tera::Tera;

/// Contains all routes for the application
pub mod routes;
/// Contains all templates for the application
pub mod components;
/// Error types & utilities
pub mod error;

mod router;

/// Path to the res directory
pub static RES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res");

lazy_static! {
    /// Tera instance
    pub static ref template: Tera = {
        let mut tera = Tera::new(
            &format!("{RES_PATH}/templates/**/*.html")
        ).unwrap_or_else(|e| {
            panic!("Failed to load templates: {}", e);
        });
        tera.autoescape_on(vec![".html"]);                
        tera
    };
}

/// Prelude module. Contains commonly used imports
pub mod prelude {
    pub use crate::{
        template,
        RES_PATH,
        router::{Router, MountRouter},
    };
    pub use tera::{
        Tera,
        Context as TeraContext
    };       
}

