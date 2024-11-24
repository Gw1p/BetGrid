use anyhow::{Result};
use crate::market_specifiers::HomeAwaySide;
use crate::grid::Grid;

pub struct AsianHandicap{}
impl AsianHandicap {
    pub fn get_payoff_grid(side: HomeAwaySide, handicap: f32, grid_size: usize) -> Result<Grid> {
        let mut payoff_grid = Grid::new(grid_size);

        match side {
            HomeAwaySide::Home => {
                AsianHandicap::set_payoff(&mut payoff_grid, handicap, grid_size, true)?
            },
            HomeAwaySide::Away => {
                AsianHandicap::set_payoff(&mut payoff_grid, handicap, grid_size, false)?
            },
        };
        Ok(payoff_grid)
    }

    fn set_payoff(payoff_grid: &mut Grid, handicap: f32, grid_size: usize, is_home: bool) -> Result<()> {
        for home_goals in 0..grid_size {
            for away_goals in 0..grid_size {
                let adjusted_score: f32 = if is_home { home_goals as f32 - away_goals as f32 + handicap } else { away_goals as f32 - home_goals as f32 + handicap };

                if handicap == handicap.floor() {
                    // round handicap
                    payoff_grid.set_payoff(home_goals, away_goals, AsianHandicap::get_handicap_payoff(adjusted_score))?;

                } else if handicap.abs() % 0.5 == 0.0 {
                    // .5 handicap
                    payoff_grid.set_payoff(home_goals, away_goals, AsianHandicap::get_handicap_payoff(adjusted_score))?;

                }  else {
                    // quarter handicap

                    let lower_half_handicap = handicap - 0.25;
                    let upper_half_handicap = handicap + 0.25;

                    let lower_adjusted_score = match is_home {
                        true => home_goals as f32 - away_goals as f32 + lower_half_handicap,
                        false => away_goals as f32 - home_goals as f32 + lower_half_handicap
                    };
                    let upper_adjusted_score = match is_home {
                        true => home_goals as f32 - away_goals as f32 + upper_half_handicap,
                        false => away_goals as f32 - home_goals as f32 + upper_half_handicap
                    };

                    let payoff = AsianHandicap::get_handicap_payoff(lower_adjusted_score) * 0.5 + AsianHandicap::get_handicap_payoff(upper_adjusted_score) * 0.5;
                    payoff_grid.set_payoff(home_goals, away_goals, payoff)?;
                }
            }
        }

        Ok(())
    }

    fn get_handicap_payoff(adjusted_score: f32) -> f64 {
        // handles round and .5 handicaps. quarter handicaps are a combination of these
        if adjusted_score > 0.0 {
            return 1.0;
        } else if adjusted_score == 0.0 {
            return 0.0;
        } else {
            return -1.0;
        }
    }

}
