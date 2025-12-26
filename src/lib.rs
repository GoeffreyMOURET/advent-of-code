// Configure rustdoc.
#![doc(html_logo_url = "https://maneatingape.github.io/advent-of-code-rust/logo.png")]

macro_rules! library {
    ($library_name:tt $description:literal $($paquet:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $library_name {
            $(pub mod $paquet;)*
        }
    }
}

library!(utils "Utilitaires plutôt pratiques"
    array_utils,
    min_max_utils
);

library!(structures "Structures de l'advent of code"
    day_trait,
    resultat_jour
);

library!(services "Services de l'advent of code"
    execution_jour_service
);

library!(aoc2025 "Advent of code 2025"
    day1, day1_input_file,
    day3, day3_input_file,
    day6, day6_input_file,
    day7, day7_input_file,
    day8, day8_input_file,
    day9, day9_input_file,
    day10, day10_input_file,
    day12, day12_input_file
);