pub mod ai;
pub mod player;

pub(self) mod name_generation {
    use rand::seq::IteratorRandom;
    use rust_embed::RustEmbed;

    const NAMES_FILE: &str = "names.txt";

    #[derive(RustEmbed)]
    #[folder = "static/resources"]
    #[include = "*.txt"]
    struct Resource;

    pub(super) fn get_random_name() -> String {
        let file_bytes = Resource::get(NAMES_FILE).expect("No such dictionary exists!");
        let file_string = String::from_utf8(file_bytes.data.as_ref().to_vec()).unwrap_or_default();

        file_string
            .lines()
            .choose(&mut rand::thread_rng())
            .unwrap() // safe since the NAMES_FILE text file is not empty
            .to_string()
    }
}
