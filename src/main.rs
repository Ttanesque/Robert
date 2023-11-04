use std::fs;

use henri::parser;
use log::debug;

fn main() {
    env_logger::init();
    debug!("Lecture du fichier css");
    let raw_css = fs::read_to_string("test/style.css").expect("fichier non trouvé :(");
    let _ = parser::parse(raw_css.as_str());
}
