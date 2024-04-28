use std::path::PathBuf;
use tarjama::Translator;

pub fn initialize_translator() -> Translator {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("translations");

    let catalogue = tarjama::loader::toml::load_sync(d).expect("couldn't load translations");

    Translator::with_catalogue_bag(catalogue)
}
