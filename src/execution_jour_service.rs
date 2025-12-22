use crate::day_trait::{Day, InputFile};
use crate::resultat_jour::ResultatJour;

pub struct ExecutionJourService;

impl ExecutionJourService {
    pub fn executer_jour(
        day: impl Day,
        input_file: InputFile,
    ) {
        println!("Executor Partie 1 : {}", day.get_description());
        let resultat = ResultatJour {
            exemple: day.executer_partie1(input_file.exemple),
            inconnu: day.executer_partie1(input_file.inconnu),
        };
        println!("{resultat}");

        println!("Executor Partie 2 : {}", day.get_description());
        let resultat = ResultatJour {
            exemple: day.executer_partie2(input_file.exemple),
            inconnu: day.executer_partie2(input_file.inconnu),
        };
        println!("{resultat}");
    }
}