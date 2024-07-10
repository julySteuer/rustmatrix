extern crate confy;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Configuation {
    pub renderer: String,
    pub state: String
}

impl Default for Configuation {
    fn default() -> Self {
        Configuation { renderer: String::from("Debug"), state: String::from("Debug") }
    }
}

pub fn get_configuration() -> Configuation {
    println!("{:#?}", confy::get_configuration_file_path("cmatrix", None));
    confy::load("cmatrix", None).expect("Configguration Error")
}