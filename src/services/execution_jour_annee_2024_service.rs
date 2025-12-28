use crate::structures::day_trait::{Day, ExecutionAnnee};

pub struct ExecutionJourAnnee2024Service;
impl ExecutionAnnee for ExecutionJourAnnee2024Service {
    fn recuperer_jour(numero_jour: u8) -> Box<dyn Day> {
        match numero_jour {
            _ => panic!("Non implémenté"),
        }
    }
}