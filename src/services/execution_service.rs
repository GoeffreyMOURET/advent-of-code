use crate::services::execution_jour_annee_2024_service::ExecutionJourAnnee2024Service;
use crate::services::execution_jour_annee_2025_service::ExecutionJourAnnee2025Service;
use crate::structures::day_trait::{Day, ExecutionAnnee};
use crate::structures::resultat_jour::ResultatJour;

pub struct ExecutionService {}
impl ExecutionService {

    pub fn executer_aoc(
        annee: u16,
        jour: u8,
    ) {
        let day: Box<dyn Day> = match annee {
            2025 => ExecutionJourAnnee2025Service::recuperer_jour(jour),
            2024 => ExecutionJourAnnee2024Service::recuperer_jour(jour),
            _ => panic!("Année non implémentée"),
        };
        Self::executer_jour(day);
    }
    fn executer_jour(
        jour: Box<dyn Day>
    ) {
        println!("Executor Partie 1 : {}", jour.get_description());
        let input_file = jour.recuperer_input_file();
        let resultat = ResultatJour {
            exemple: jour.executer_partie1(input_file.exemple),
            inconnu: jour.executer_partie1(input_file.inconnu),
        };
        println!("{resultat}");

        println!("Executor Partie 2 : {}", jour.get_description());
        let input_file = jour.recuperer_input_file_partie2();
        let resultat = ResultatJour {
            exemple: jour.executer_partie2(input_file.exemple),
            inconnu: jour.executer_partie2(input_file.inconnu),
        };
        println!("{resultat}");
    }

}