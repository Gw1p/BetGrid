
use anyhow::{Result, bail};
use crate::bet_types::win_draw_win::{WinDrawWin};
use crate::bet_types::asian_handicap::{AsianHandicap};
use crate::bet_types::over_under::{OverUnder};
use crate::market_specifiers::{HomeAwayDrawSide, HomeAwaySide, OverUnderSide};
use std::str::FromStr;
use serde_json;
use crate::error::{Error};
use crate::config::DEFAULT_FB_GRID_SIZE;

pub struct BetGrid{}
impl BetGrid {
    pub fn new() -> Self{
        Self{}
    }

    pub fn run(&self, matches: clap::ArgMatches) -> Result<()> {
        self.payoff_grid(matches)?;
        Ok(())
    }

    fn payoff_grid(&self, matches: clap::ArgMatches) -> Result<()> {
        let cout_handler = BetGridCliHandler::new();
        cout_handler.payoff_grid(matches)?;
        Ok(())
    }
}

enum BetType {
    WinDrawWin,
    AsianHandicap,
    OverUnder
}

pub enum OutputMode {
    Text,
    Json
}

struct BetGridCliHandler{}
impl BetGridCliHandler {
    fn new() -> Self {
        Self{}
    }

    fn payoff_grid(&self, matches: clap::ArgMatches) -> Result<()> {
        // Figure out if we're outputting text of json
        let output_mode = self.get_output_mode(matches.clone());
        let grid_size: usize = self.get_grid_size(matches.clone())?;

        // get a BetTypes enum from the given bet type argument
        let bet_type = self.get_bet_type(matches.clone(), &output_mode)?;

        // For each bet type, extract any extra params and call the appropriate grid method
        match bet_type {
            BetType::WinDrawWin => {
                let side_str = matches.get_one::<String>("side");
                match side_str {
                    Some(matched_side) => {
                        let side = HomeAwayDrawSide::from_str(matched_side);
                        let grid = WinDrawWin::get_payoff_grid(side.unwrap(), grid_size)?;
                        grid.print(&output_mode)?;
                    },
                    None => {
                        self.output_error(&output_mode, "Couldn't find required argument 'side' for win-draw-win bet type. Should be one of: 'home', 'away', 'draw' (or 'h', 'a', 'd').".to_owned())?;
                        return Ok(())
                    }
                }
            },

            BetType::AsianHandicap => {
                let side = match matches.get_one::<String>("side") {
                    Some(matched_side) => {
                        HomeAwaySide::from_str(matched_side)
                    },
                    None => {
                        self.output_error(&output_mode, "Couldn't find required argument 'side' for asian-handicap bet type. Should be one of: 'home' or 'away' ('h' or 'a').".to_owned())?;
                        return Ok(())
                    }
                };
                let handicap: f32 = match matches.get_one::<String>("handicap") {
                    Some(matched_handicap) => matched_handicap.parse()?,
                    None => {
                        self.output_error(&output_mode, "Couldn't find required argument 'handicap' for asian-handicap bet type. For example, -0.25, 1, 1.5.".to_owned())?;
                        return Ok(())
                    }
                };
                let grid = AsianHandicap::get_payoff_grid(side.unwrap(), handicap, grid_size)?;
                grid.print(&output_mode)?;
            },

            BetType::OverUnder => {
                let side = match matches.get_one::<String>("side") {
                    Some(matched_side) => {
                        OverUnderSide::from_str(matched_side)
                    },
                    None => {
                        self.output_error(&output_mode, "Couldn't find required argument 'side' for over-under bet type. Should be one of: 'over' or 'under' ('o' or 'u').".to_owned())?;
                        return Ok(())
                    }
                };
                let goals: f32 = match matches.get_one::<String>("goals") {
                    Some(matched_goals) => matched_goals.parse()?,
                    None => {
                        self.output_error(&output_mode, "Couldn't find required argument 'goals' for over-under bet type. For example, 1, 1.5, 2.".to_owned())?;
                        return Ok(())
                    }
                };
                let grid = OverUnder::get_payoff_grid(side.unwrap(), goals, grid_size)?;
                grid.print(&output_mode)?;
            },
        }

        Ok(())
    }

    fn output_error(&self, output_mode: &OutputMode, error_message: String) -> Result<()> {
        match output_mode {
            OutputMode::Text => bail!(error_message),
            OutputMode::Json => {
                let error = Error{error: error_message};
                let json_error = serde_json::to_string_pretty(&error);
                match json_error {
                    Ok(serialized_error) => bail!(serialized_error),
                    _ => bail!("{{\"error\": \"unexpected_error\"}}")
                }
            }
        };
    }

    fn get_output_mode(&self, matches: clap::ArgMatches) -> OutputMode {
        match matches.get_one::<String>("output").map(String::as_str) {
            Some("json") | Some("j") => OutputMode::Json,
            _ => OutputMode::Text
        }
    }

    fn get_grid_size(&self, matches: clap::ArgMatches) -> Result<usize> {
        let size: usize = match matches.get_one::<String>("grid_size").map(String::as_str) {
            Some(grid_size) => grid_size.parse()?,
            _ => DEFAULT_FB_GRID_SIZE
        };
        Ok(size)
    }

    fn get_bet_type(&self, matches: clap::ArgMatches, output_mode: &OutputMode) -> Result<BetType> {
        let bet_type_input = matches.get_one::<String>("bet_type");
        match bet_type_input {
            None => {
                self.output_error(&output_mode, "Couldn't find required argument 'bet_type'. Must be one of 'win-draw-win', 'asian-handicap' or 'over-under'.".to_owned())?;
                // dummy return, the above always errors
                return Ok(BetType::WinDrawWin)
            },
            Some(matched_bet_type) => {
                match matched_bet_type.as_str() {
                    "win-draw-win" => Ok(BetType::WinDrawWin),
                    "asian-handicap" => Ok(BetType::AsianHandicap),
                    "over-under" => Ok(BetType::OverUnder),
                    other => {
                        let error_message = format!("Unsupported bet type '{}'", other);
                        self.output_error(&output_mode, error_message)?;
                        // dummy return, the above always errors
                        return Ok(BetType::WinDrawWin)
                    }
                }
            }
        }
    }
}