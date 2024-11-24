use anyhow::{Result};
use crate::market_specifiers::HomeAwayDrawSide;
use crate::grid::Grid;

pub struct WinDrawWin{}
impl WinDrawWin {
    pub fn get_payoff_grid(side: HomeAwayDrawSide, grid_size: usize) -> Result<Grid> {
        let mut payoff_grid = Grid::new(grid_size);

        match side {
            HomeAwayDrawSide::Home => WinDrawWin::set_payoff(&mut payoff_grid, grid_size, 1.0, -1.0, -1.0)?,
            HomeAwayDrawSide::Away => WinDrawWin::set_payoff(&mut payoff_grid, grid_size, -1.0, 1.0, -1.0)?,
            HomeAwayDrawSide::Draw => WinDrawWin::set_payoff(&mut payoff_grid, grid_size, -1.0, -1.0, 1.0)?
        };
        Ok(payoff_grid)
    }

    fn set_payoff(payoff_grid: &mut Grid, grid_size: usize, home_payoff: f64, away_payoff: f64, draw_payoff: f64) -> Result<()> {
        for home_goals in 0..grid_size {
            for away_goals in 0..grid_size {
                if home_goals > away_goals {
                    payoff_grid.set_payoff(home_goals, away_goals, home_payoff)?;

                } else if away_goals > home_goals {
                    payoff_grid.set_payoff(home_goals, away_goals, away_payoff)?;

                } else {
                    // draw
                    payoff_grid.set_payoff(home_goals, away_goals, draw_payoff)?;

                }
            }
        }

        Ok(())
    }
}
