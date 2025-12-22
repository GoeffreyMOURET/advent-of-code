pub trait Day {
    fn get_description(&self) -> String;
    fn executer_partie1(&self, input: &str) -> i128;
    fn executer_partie2(&self, input: &str) -> i128;

}

pub struct InputFile {
    pub exemple: &'static str,
    pub inconnu: &'static str
}

