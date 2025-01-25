use crate::grid::Grid;
use crate::market_specifiers::HomeAwayDrawSide;
use anyhow::Result;

pub struct WinDrawWin {}
impl WinDrawWin {
    pub fn get_payoff_grid(side: HomeAwayDrawSide, grid_size: usize) -> Result<Grid> {
        let mut payoff_grid = Grid::new(grid_size);

        match side {
            HomeAwayDrawSide::Home => {
                WinDrawWin::set_payoff(&mut payoff_grid, grid_size, 1.0, -1.0, -1.0)?
            }
            HomeAwayDrawSide::Away => {
                WinDrawWin::set_payoff(&mut payoff_grid, grid_size, -1.0, 1.0, -1.0)?
            }
            HomeAwayDrawSide::Draw => {
                WinDrawWin::set_payoff(&mut payoff_grid, grid_size, -1.0, -1.0, 1.0)?
            }
        };
        Ok(payoff_grid)
    }

    fn set_payoff(
        payoff_grid: &mut Grid,
        grid_size: usize,
        home_payoff: f64,
        away_payoff: f64,
        draw_payoff: f64,
    ) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_payoff_grid<F>(side: HomeAwayDrawSide, payoff_calc: F)
    where
        F: Fn(f64) -> f64,
    {
        let wdw = WinDrawWin::get_payoff_grid(side.clone(), 10).unwrap();
        let payoff_grid = wdw.print_json().unwrap().payoff_grid;

        for home_goals in 0..10 {
            for away_goals in 0..10 {
                let expected_payoff = payoff_calc(home_goals as f64 - away_goals as f64);
                if let Some(inner_grid) = payoff_grid.get(&home_goals) {
                    if let Some(actual_payoff) = inner_grid.get(&away_goals) {
                        assert_eq!(
                            actual_payoff, &expected_payoff,
                            "Have wdw {}. Expect {}:{} payoff to be {} but got {}",
                            side, home_goals, away_goals, expected_payoff, actual_payoff
                        );
                    } else {
                        panic!("Couldn't find payoff for {} away goals", home_goals);
                    }
                } else {
                    panic!("Couldn't find payoff for {} home goals", home_goals);
                }
            }
        }
    }

    #[test]
    fn test_home_payoff() {
        test_payoff_grid(HomeAwayDrawSide::Home, |result: f64| match result {
            result if result > 0.0 => 1.0,
            _ => -1.0,
        });
    }

    #[test]
    fn test_away_payoff() {
        test_payoff_grid(HomeAwayDrawSide::Away, |result: f64| match result {
            result if result < 0.0 => 1.0,
            _ => -1.0,
        });
    }

    #[test]
    fn test_draw_payoff() {
        test_payoff_grid(HomeAwayDrawSide::Draw, |result: f64| match result {
            0.0 => 1.0,
            _ => -1.0,
        });
    }
}
