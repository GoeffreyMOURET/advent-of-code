use std::fmt::{Display, Formatter};

pub struct ResultatJour{
    pub exemple: i128,
    pub inconnu: i128,
}

impl Display for ResultatJour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Exemple : {}, Inconnu : {}",
                self.exemple.to_string(),
                self.inconnu.to_string()
            )

    }
}