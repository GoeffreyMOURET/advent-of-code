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
    min_max_utils,
    dfs_utils
);

library!(structures "Structures de l'advent of code"
    day_trait,
    resultat_jour
);

library!(services "Services de l'advent of code"
    execution_jour_service
);

macro_rules! library_day {
    (
        $library_name:ident,
        $description:literal,
        [$($day_mod:ident),*],
        [$($input_mod:ident),*]
    ) => {
        #[doc = concat!("# ", $description)]
        pub mod $library_name {
            pub mod day {
                $( pub mod $day_mod; )*
            }

            pub mod input {
                $( pub mod $input_mod; )*
            }
        }
    };
}

library_day!(
    aoc2025,
    "Advent of code 2025",
    [day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12],
    [day1_input_file, day2_input_file, day3_input_file, day4_input_file,
     day5_input_file, day6_input_file, day7_input_file, day8_input_file,
     day9_input_file, day10_input_file, day11_input_file, day12_input_file]
);