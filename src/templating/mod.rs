use std::path::PathBuf;

use tera::Tera;

pub mod filter;

pub fn initialize_engine() -> tera::Tera {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("translations");

    match Tera::new("templates/**/*.html") {
        Ok(mut t) => {
            t.autoescape_on(vec![".html"]);

            return t;
        }
        Err(e) => {
            println!("Parsing error(s): {}", e);

            ::std::process::exit(1);
        }
    }
}
