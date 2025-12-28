pub trait Day {
    fn get_description(&self) -> String;
    fn executer_partie1(&self, input: &str) -> i128;
    fn executer_partie2(&self, input: &str) -> i128;
    fn recuperer_input_file(&self) -> InputFile;
    fn recuperer_input_file_partie2(&self) -> InputFile {
        self.recuperer_input_file()
    }

}

pub struct InputFile {
    pub exemple: &'static str,
    pub inconnu: &'static str
}

pub trait ExecutionAnnee {
    fn recuperer_jour(numero_jour: u8) -> Box<dyn Day>;
}
