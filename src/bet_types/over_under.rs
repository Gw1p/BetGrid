use anyhow::{Result};
use crate::market_specifiers::OverUnderSide;
use crate::grid::Grid;

pub struct OverUnder{}
impl OverUnder {
    pub fn get_payoff_grid(side: OverUnderSide, goals: f32, grid_size: usize) -> Result<Grid> {
        let mut payoff_grid = Grid::new(grid_size);

        match side {
            OverUnderSide::Over => OverUnder::set_payoff(&mut payoff_grid, grid_size, goals, 1.0, -1.0)?,
            OverUnderSide::Under => OverUnder::set_payoff(&mut payoff_grid, grid_size, goals, -1.0, 1.0)?,
        };
        Ok(payoff_grid)
    }

    fn set_payoff(payoff_grid: &mut Grid, grid_size: usize, goals: f32, over_payoff: f64, under_payoff: f64) -> Result<()> {
        for home_goals in 0..grid_size {
            for away_goals in 0..grid_size {
                let total_goals = (home_goals + away_goals) as f32;

                if goals == goals.floor() || goals.abs() % 0.5 == 0.0 {
                    // round or .5
                    payoff_grid.set_payoff(home_goals, away_goals, OverUnder::get_payoff(goals, total_goals, over_payoff, under_payoff))?;

                } else {
                    // quarter
                    let goals_lower = goals - 0.25;
                    let goals_upper = goals + 0.25;

                    let lower_payoff = OverUnder::get_payoff(goals_lower, total_goals, over_payoff, under_payoff);
                    let upper_payoff = OverUnder::get_payoff(goals_upper, total_goals, over_payoff, under_payoff);

                    let payoff = lower_payoff * 0.5 + upper_payoff * 0.5;
                    payoff_grid.set_payoff(home_goals, away_goals, payoff)?;
                }
            }
        }

        Ok(())
    }

    fn get_payoff(target_goals: f32, total_goals: f32, over_payoff: f64, under_payoff: f64) -> f64 {
        if total_goals > target_goals {
            return over_payoff;
        } else if total_goals < target_goals {
            return under_payoff;
        } else {
            return 0.0;
        }
    }
}
