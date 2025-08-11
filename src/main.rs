use clap::Parser;
use rand::seq::IteratorRandom;
use rust_solver::*;

fn print_possible_jesus(assignments: Vec<Vec<bool>>, players: &Vec<Player>) {
    println!("\n Possible Jesus: {}", assignments.len());
    for (index, assignment) in assignments.iter().enumerate() {
        let candidate_id = assignment
            .iter()
            .position(|&v| v == true)
            .expect("There should be at least one Jesus assignment");
        println!(
            "Model {}: Jesus -> {:#?}",
            index,
            players[candidate_id].name()
        );
    }
}

fn validate_players(s: &str) -> Result<usize, String> {
    match s.parse::<usize>() {
        Ok(val) if (4..=10).contains(&val) => Ok(val),
        _ => Err(String::from("Players must be between 4 and 10")),
    }
}

fn main() {
    #[derive(Parser)]
    struct Args {
        #[clap(
            short,
            long,
            default_value = "7",
            help = "The number of players to simulate in the game.",
        value_parser = validate_players )]
        players: usize,

        #[clap(
            short,
            long,
            help = "Optional random seed for reproducibility.",
            default_value = "0"
        )]
        seed: u64,

        #[clap(
            short,
            long,
            help = "Enable verbose output for detailed game simulation."
        )]
        verbose: bool,
    }

    let args = Args::parse();
    let number_of_players = args.players;
    let seed = args.seed;
    let verbose = args.verbose;
    let mut players = get_players(number_of_players);
    let events = get_events(number_of_players);

    let mut jesus_finder = JesusFinder::new(number_of_players);
    print_possible_jesus(jesus_finder.find_jesus(), &players);

    let rng = &mut rand::rng();

    for (event_index, event) in events.iter().enumerate() {
        println!("\nEvent #{}: {:#?}", event_index + 1, event);
        let selected_player_ids: Vec<usize> =
            (0..players.len()).choose_multiple(rng, event.participation());

        if verbose {
            println!(
                "Selected players: {:#?}",
                selected_player_ids
                    .iter()
                    .map(|&id| players[id].name())
                    .collect::<Vec<_>>()
            );
        }

        let selected_prodigies = selected_player_ids
            .iter()
            .map(|&id| {
                let player = &mut players[id];

                let prodigy = player.consume_prodigy(event_index);
                if verbose {
                    println!("{} picked {:?}={}", player.name(), prodigy, prodigy.value());
                }
                prodigy
            })
            .collect::<Vec<_>>();

        let total = selected_prodigies.iter().map(|p| p.value()).sum::<i32>();
        if verbose {
            println!(
                "Total value: {}, Difficulty: {} --> Passed: {}\n",
                total,
                event.difficulty(),
                total >= event.difficulty() as i32
            );
        }

        jesus_finder.process_event(
            event_index,
            selected_player_ids.as_slice(),
            selected_prodigies.as_slice(),
        );

        let models = jesus_finder.find_jesus();
        assert!(models.len() >= 1);
        if verbose {
            print_possible_jesus(models, &players);
        }
    }
    println!(
        "\nSimulation complete: {} players - seed {}",
        number_of_players, seed
    );
    print_possible_jesus(models, &players);
}
