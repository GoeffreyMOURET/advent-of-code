use crate::aoc2025::day::day10::generateur_etat::GenerateurEtat;
use crate::aoc2025::input::day10_input_file::INPUT_FILE_DAY10;
use crate::structures::day_trait::{Day, InputFile};

pub struct Day10 {}
impl Day for Day10 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 10")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        input.split('\n')
           .map(InfoMachine::parse)
           .map(|info_machine| generateur_etat::GenerateurEtat::new(info_machine))
           .map(GenerateurEtat::trouver_plus_petit_solveur_part1)
           .sum::<i128>()

    }

    fn executer_partie2(&self, input: &str) -> i128 {
        input.split('\n')
            .map(InfoMachine::parse)
            .map(|info_machine| generateur_etat::GenerateurEtat::new(info_machine))
            .map(GenerateurEtat::trouver_plus_petit_solveur_part2)
            .sum::<u64>() as i128
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY10
    }
}

impl Day10 {

}

mod generateur_etat {
    use crate::aoc2025::day::day10::InfoMachine;

    #[derive(Debug)]
    pub struct GenerateurEtat {
        info_machine: InfoMachine,
        etat_en_cours: Vec<usize>,
    }

    impl GenerateurEtat {
        pub fn new(info_machine: InfoMachine) -> GenerateurEtat {
            debug_assert!(info_machine.joltage.len() == info_machine.cible.len(), "Erreur lors du parsing");
            GenerateurEtat { info_machine, etat_en_cours: vec![] }
        }

        pub fn trouver_plus_petit_solveur_part1(mut self) -> i128 {
            while !self.info_machine.est_ok_part1() {
                self.update_etat_suivant();
                self.info_machine.appuyer_boutons(&self.etat_en_cours);
            }
            self.etat_en_cours.len() as i128
        }

        pub fn trouver_plus_petit_solveur_part2(self) -> u64 {
            /* NE MARCHE PAS - Nécessite d'avoir une lib z3 fonctionelle - insupprotable à intégrer sur Windows
            let inconnues = self.info_machine.actions.iter().enumerate()
                .map(|(index, a)| { Int::fresh_const(index.to_string().trim())})
                .collect::<Vec<Int>>();
            let solver = Solver::new();
            inconnues.iter().for_each(|inconnue| {
                solver.assert(&inconnue.gt(-1));
            });
            for (index, j) in self.info_machine.joltage.iter().enumerate() {
                let mut equation: Option<Int> = None;
                for i in 0..inconnues.len() {
                    if self.info_machine.actions[i].contains(&index) {
                        equation = Option::from(
                            equation.map(|eq| { inconnues[i].clone() + eq })
                                .unwrap_or(inconnues[i].clone())
                        );
                    }

                }
                solver.assert(&(equation.unwrap()).eq(*j as u64));
            }

            solver.solutions(inconnues, false)
                .take(1000)
                .map(|solution| solution.iter().map(Int::as_u64).map(Option::unwrap).collect::<Vec<u64>>())
                .map(|solution| { solution.iter().sum() })
                .min().unwrap_or(0)
             */
            0
        }

        /*
            Seuls les états dans l'ordre croissant sont gardés
            (ça permet d'éviter de tester les mêmes états car l'ordre des boutons n'est pas intéressant)
         */
        fn update_etat_suivant(&mut self) {
            let nb_boutons = self.info_machine.actions.len();
            if self.etat_en_cours.iter().all(|v| *v == nb_boutons - 1) {
                self.etat_en_cours = vec![0; self.etat_en_cours.len() + 1];
                return;
            }
            let mut nouvel_etat = self.etat_en_cours.clone();
            for (index, etat) in self.etat_en_cours.iter().enumerate().rev() {
                debug_assert!(*etat < nb_boutons, "Impossible etat: {} nb_boutons: {}", etat, nb_boutons);
                if nouvel_etat[index] < nb_boutons - 1 {
                    nouvel_etat[index] = *etat + 1;
                    break;
                } else {
                    nouvel_etat[index] = nouvel_etat[index-1];
                }
            }
            self.etat_en_cours = nouvel_etat;
        }
    }
}

#[derive(Debug)]
struct InfoMachine {
    cible: Vec<bool>,
    actions: Vec<Vec<usize>>,
    joltage: Vec<usize>,
    etat: Vec<bool>,
}

impl InfoMachine {
    pub fn parse(content: &str) -> InfoMachine {
        let mut content_split: Vec<&str> = content.trim().split(" ").collect();
        let cible_str = content_split.remove(0);
        let joltage_str = content_split.remove(content_split.len() - 1);
        let actions_str = content_split;
        InfoMachine {
            cible: cible_str.chars()
                .filter(|c| { *c == '.' || *c == '#' })
                .map(|c| { c == '#' })
                .collect(),
            actions: actions_str.iter()
                .map(|a| { Self::conserver_chiffres_et_virgules(a) } )
                .map(|a| { a.split(',').filter_map(|c| { c.parse::<usize>().ok() }).collect() })
                .collect(),
            joltage: Self::conserver_chiffres_et_virgules(&joltage_str).split(',')
                .filter_map(|c| { c.parse::<usize>().ok() })
                .collect(),
            etat: vec![false; cible_str.len()],
        }
    }

    pub fn est_ok_part1(&self) -> bool {
        self.cible.iter()
            .zip(self.etat.iter())
            .all(|(c, e)| *c == *e)
    }

    pub fn appuyer_boutons(&mut self, boutons: &Vec<usize>) {
        self.etat = vec![false; self.cible.len()];
        for bouton in boutons {
            let actions_a_realiser = &self.actions[*bouton];
            for action in actions_a_realiser {
                self.etat[*action] = !self.etat[*action];
            }
        }
    }

    fn conserver_chiffres_et_virgules(a: &&str) -> String {
        a.chars().filter(|c| { c.is_ascii_digit() || *c == ',' }).collect()
    }
}
