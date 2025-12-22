use crate::day_trait::{Day};
use crate::resultat_jour::ResultatJour;

pub struct ExecutionJourService;

impl ExecutionJourService {
    pub fn executer_jour(
        day: Box<dyn Day>
    ) {
        println!("Executor Partie 1 : {}", day.get_description());
        let input_file = day.recuperer_input_file();
        let resultat = ResultatJour {
            exemple: day.executer_partie1(input_file.exemple),
            inconnu: day.executer_partie1(input_file.inconnu),
        };
        println!("{resultat}");

        println!("Executor Partie 2 : {}", day.get_description());
        let input_file = day.recuperer_input_file_partie2();
        let resultat = ResultatJour {
            exemple: day.executer_partie2(input_file.exemple),
            inconnu: day.executer_partie2(input_file.inconnu),
        };
        println!("{resultat}");
    }
}