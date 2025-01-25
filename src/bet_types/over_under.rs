use crate::grid::Grid;
use crate::market_specifiers::OverUnderSide;
use anyhow::Result;

pub struct OverUnder {}
impl OverUnder {
    pub fn get_payoff_grid(side: OverUnderSide, goals: f32, grid_size: usize) -> Result<Grid> {
        let mut payoff_grid = Grid::new(grid_size);

        match side {
            OverUnderSide::Over => {
                OverUnder::set_payoff(&mut payoff_grid, grid_size, goals, 1.0, -1.0)?
            }
            OverUnderSide::Under => {
                OverUnder::set_payoff(&mut payoff_grid, grid_size, goals, -1.0, 1.0)?
            }
        };
        Ok(payoff_grid)
    }

    fn set_payoff(
        payoff_grid: &mut Grid,
        grid_size: usize,
        goals: f32,
        over_payoff: f64,
        under_payoff: f64,
    ) -> Result<()> {
        for home_goals in 0..grid_size {
            for away_goals in 0..grid_size {
                let total_goals = (home_goals + away_goals) as f32;

                if goals == goals.floor() || goals.abs() % 0.5 == 0.0 {
                    // round or .5
                    payoff_grid.set_payoff(
                        home_goals,
                        away_goals,
                        OverUnder::get_payoff(goals, total_goals, over_payoff, under_payoff),
                    )?;
                } else {
                    // quarter
                    let goals_lower = goals - 0.25;
                    let goals_upper = goals + 0.25;

                    let lower_payoff =
                        OverUnder::get_payoff(goals_lower, total_goals, over_payoff, under_payoff);
                    let upper_payoff =
                        OverUnder::get_payoff(goals_upper, total_goals, over_payoff, under_payoff);

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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_payoff_grid<F>(side: OverUnderSide, goals: f32, payoff_calc: F)
    where
        F: Fn(f64) -> f64,
    {
        let ou = OverUnder::get_payoff_grid(side.clone(), goals, 10).unwrap();
        let payoff_grid = ou.print_json().unwrap().payoff_grid;

        for home_goals in 0..10 {
            for away_goals in 0..10 {
                let expected_payoff = payoff_calc(home_goals as f64 + away_goals as f64);
                if let Some(inner_grid) = payoff_grid.get(&home_goals) {
                    if let Some(actual_payoff) = inner_grid.get(&away_goals) {
                        assert_eq!(
                            actual_payoff, &expected_payoff,
                            "Have ou {} with {} goals. Expect {}:{} payoff to be {} but got {}",
                            side, goals, home_goals, away_goals, expected_payoff, actual_payoff
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
    fn test_over_payoff() {
        test_payoff_grid(OverUnderSide::Over, 1.5, |result: f64| match result {
            result if result > 1.5 => 1.0,
            _ => -1.0,
        });
    }

    #[test]
    fn test_under_payoff() {
        test_payoff_grid(OverUnderSide::Under, 1.5, |result: f64| match result {
            result if result < 1.5 => 1.0,
            _ => -1.0,
        });
    }

    #[test]
    fn test_over_payoff_whole() {
        test_payoff_grid(OverUnderSide::Over, 2.0, |result: f64| match result {
            result if result > 2.0 => 1.0,
            2.0 => 0.0,
            _ => -1.0,
        });
    }

    #[test]
    fn test_under_payoff_whole() {
        test_payoff_grid(OverUnderSide::Under, 2.0, |result: f64| match result {
            result if result < 1.5 => 1.0,
            2.0 => 0.0,
            _ => -1.0,
        });
    }

    #[test]
    fn test_over_payoff_quarter() {
        test_payoff_grid(OverUnderSide::Over, 2.25, |result: f64| match result {
            result if result > 2.5 => 1.0,
            2.0 => -0.5,
            _ => -1.0,
        });
    }

    #[test]
    fn test_under_payoff_quarter() {
        test_payoff_grid(OverUnderSide::Under, 2.25, |result: f64| match result {
            result if result < 1.5 => 1.0,
            2.0 => 0.5,
            _ => -1.0,
        });
    }

    #[test]
    fn test_over_payoff_quarter2() {
        test_payoff_grid(OverUnderSide::Over, 2.75, |result: f64| match result {
            3.0 => 0.5,
            result if result > 2.5 => 1.0,
            _ => -1.0,
        });
    }

    #[test]
    fn test_under_payoff_quarter2() {
        test_payoff_grid(OverUnderSide::Under, 2.75, |result: f64| match result {
            3.0 => -0.5,
            result if result < 2.5 => 1.0,
            _ => -1.0,
        });
    }
}
