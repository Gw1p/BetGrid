use clap::{Arg, Command};
mod bet_grid_handler;
mod bet_types;
mod grid;
mod market_specifiers;
mod config;
mod error;

fn main() {
    let matches = Command::new("Football Betting Payoff Grid")
        .version("1.0")
        .author("Anton 'Gw1p' Bendrikov")
        .about("CLI tool that displays payoff grids for football bet types")
        .arg(
            Arg::new("bet_type")
            .long("bet-type")
            .required(true)
            .help("The type of bet (e.g. win-draw-win, asian-handicap, over-under)")
        )
        .arg(Arg::new("output")
            .long("output")
            .help("Output format (json, text). Defaults to text.")
        )
        .arg(Arg::new("grid_size")
            .long("grid_size")
            .help("Grid size (numeric). Defaults to 10 (starting from 0).")
        )
        .arg(
            Arg::new("side")
            .long("side")
            .help("Bet type 'side'. For win-draw-win, this is 'home', 'away', 'draw'. For overs/unders this is 'over' and 'under'.")
        )
        .arg(Arg::new("handicap")
            .long("handicap")
            .help("Handicap for Asian Handicap bets (for example, -1.5).")
        ).arg(Arg::new("goals")
            .long("goals")
            .help("Goals for Over Under bets (for example, 1 or 1.5).")
        ).get_matches();
    match bet_grid_handler::BetGrid::new().run(matches) {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}
