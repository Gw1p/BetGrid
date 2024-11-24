use std::collections::HashMap;

use anyhow::{Result, bail};
use serde::Serialize;
use serde_json;
use colored::*;

use crate::bet_grid_handler::{OutputMode};


pub struct Grid {
    grid: Vec<Vec<f64>>,
    grid_size: usize
}

#[derive(Serialize)]
struct JsonGrid {
    grid_size: usize,
    payoff_grid: HashMap<usize, HashMap<usize, f64>>
}

impl Grid {
    pub fn new(grid_size: usize) -> Self {
        let size = grid_size.try_into().unwrap();
        Self{ grid: vec![vec![0.0; size]; size], grid_size: size }
    }

    pub fn set_payoff(&mut self, idx_x: usize, idx_y: usize, payoff: f64) -> Result<()> {
        if !self.is_in_bounds(idx_x, idx_y) {
            bail!("Got invalid x or y coordinate: {} {}. It must be between 0 and {}.", idx_x, idx_y, self.grid_size);
        }

        self.grid[idx_x][idx_y] = payoff;
        Ok(())
    }

    fn is_in_bounds(&self, idx_x: usize, idx_y: usize) -> bool {
        idx_x < self.grid_size && idx_y < self.grid_size
    }

    pub fn print(&self, output_mode: &OutputMode) -> Result<()> {
        match output_mode {
            OutputMode::Text => self.print_text()?,
            OutputMode::Json => self.print_json()?,
        };
        Ok(())
    }

    pub fn print_text(&self) -> Result<()> {
        let col_lengths = self.get_column_lengths();
        // "+ 3" because each payoff is prepended by a space and appended by space & "|"
        let line_length: usize = 7 + col_lengths.values().map(|&v| v + 3).sum::<usize>();
        let away_start = line_length / 2 - 2;

        // Prints top line with "AWAY"
        print!("{}", " ".repeat(away_start));
        print!("AWAY");
        print!("{}", " ".repeat(away_start));
        println!();

        let num_rows = 6 + self.grid_size;
        let home_start = num_rows / 2 - 2;
        let home_end = num_rows / 2 + 2;

        let home = vec!["H", "O", "M", "E"];

        // (horizontally) print away goals
        print!("     ||");
        for away_goals in 0..self.grid_size {
            let col_length = self.get_col_length(&col_lengths, away_goals);
            let away_goals_len = away_goals.to_string().len();
            let extra_spaces = col_length.saturating_sub(away_goals_len);
            print!(" {}{} |", " ".repeat(extra_spaces), away_goals);
        }
        println!();
        // blank line separating away goals from payoff grid
        println!("{}", "-".repeat(line_length));

        // print out the grid (together with home goals)
        for home_goals in 0..self.grid_size {
            let row_idx = home_goals + 3;

            // Prints home goals (and home label)
            if row_idx >= home_start && row_idx < home_end {
                print!("{} {:>2} ||", home[row_idx - home_start], home_goals);
            } else {
                print!("  {:>2} ||", home_goals);
            }

            // Payoff
            for away_goals in 0..self.grid_size {
                let col_length = self.get_col_length(&col_lengths, away_goals);

                let payoff = self.grid[home_goals][away_goals];
                let payoff_str = payoff.to_string();
                let extra_spaces = col_length.saturating_sub(payoff_str.len());
                // println!("extra spaces {} because my payoff len is {} and total {}", extra_spaces, payoff_str.len(), col_length);
                if payoff > 0.0 {
                    print!(" {}{} |", " ".repeat(extra_spaces), payoff_str.green());
                } else if payoff < 0.0 {
                    print!(" {}{} |", " ".repeat(extra_spaces), payoff_str.red());
                } else {
                    print!(" {}{} |", " ".repeat(extra_spaces), payoff_str.white());
                }
            }
            println!();
        }

        Ok(())
    }

    fn get_column_lengths(&self) -> HashMap<usize, usize> {
        // Each column length is the longest 
        let mut goal_to_col_len = HashMap::new();
        for away_goals in 0..self.grid_size {
            let mut largest_payoff = String::new();
            for home_goals in 0..self.grid_size {
                let payoff_str = self.grid[home_goals][away_goals].to_string();
                if payoff_str.len() > largest_payoff.len() {
                    largest_payoff = payoff_str.clone();
                }
            }
            goal_to_col_len.insert(away_goals, largest_payoff.len());
        }
        return goal_to_col_len;
    }

    fn get_col_length(&self, col_lengths: &HashMap<usize, usize>, goals: usize) -> usize {
        match col_lengths.get(&goals) {
            Some(computed_col_length) => *computed_col_length,
            _ => 3,
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let mut map_grid: HashMap<usize, HashMap<usize, f64>> = HashMap::new();

        for home_goals in 0..self.grid_size {
            let mut row = HashMap::new();
            for away_goals in 0..self.grid_size {
                row.insert(away_goals, self.grid[home_goals][away_goals]);
            }
            map_grid.insert(home_goals, row);
        }

        let json_grid = JsonGrid {
            grid_size: self.grid_size,
            payoff_grid: map_grid
        };
        let json_output = serde_json::to_string_pretty(&json_grid)?;
        println!("{}", json_output);
        Ok(())
    }
}
