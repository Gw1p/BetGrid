# Bet Grid

A CLI tool to generate payoff grids for various football betting types, including win-draw-win, asian handicap, and over/under.

```bash
$> bet_grid --bet-type=win-draw-win --side=h
                          AWAY
     ||  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 |
---------------------------------------------------------
   0 || -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
   1 ||  1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
   2 ||  1 |  1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
H  3 ||  1 |  1 |  1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
O  4 ||  1 |  1 |  1 |  1 | -1 | -1 | -1 | -1 | -1 | -1 |
M  5 ||  1 |  1 |  1 |  1 |  1 | -1 | -1 | -1 | -1 | -1 |
E  6 ||  1 |  1 |  1 |  1 |  1 |  1 | -1 | -1 | -1 | -1 |
   7 ||  1 |  1 |  1 |  1 |  1 |  1 |  1 | -1 | -1 | -1 |
   8 ||  1 |  1 |  1 |  1 |  1 |  1 |  1 |  1 | -1 | -1 |
   9 ||  1 |  1 |  1 |  1 |  1 |  1 |  1 |  1 |  1 | -1 |
```

## Features

- Supports a variety of bet types:
    - `win-draw-win` with three possible outcomes: home (`h`), away (`a`), or draw (`d`).
    - `asian-handicap` for either `home` or `away` side for the given `handicap`
    - `over-under` for `over` or `under` side and the given number of `goals`
- (By default) outputs the grid in human-friendly format with colorful grid representation.
    - Supports json output mode

### Command Syntax

```bash
bet_grid --bet-type=<BET_TYPE> [OPTIONS]
```

Every bet type comes with their own arguments. For example, `asian-handicap` requires `side` and `handicap` arguments to be passed in.

If an argument is missing, the program will tell you.

### Arguments

--bet-type: Type of bet. Accepted values: win-draw-win, asian-handicap, over-under.
- For win-draw-win, requires --side argument with h for home, a for away, or d for draw.
- For asian-handicap requires --side with home or away, and --handicap with a numeric value.
- For over-under requires --side with over or under, and --goals with a numeric goal value.
- --output: Choose the output format. Possible values: text (default) or json (output in JSON format).

### Examples

#### Win-Draw-Win

```bash
bet_grid --bet-type=win-draw-win --side=h
bet_grid --bet-type=win-draw-win --side=a
bet_grid --bet-type=win-draw-win --side=d
```

Output: Displays a grid showing win-draw-win outcomes for home (h), away (a), or draw (d).

Example output for `--side=h`:
```bash
                          AWAY
     ||  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 |
---------------------------------------------------------
   0 || -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
   1 ||  1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
   2 ||  1 |  1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
H  3 ||  1 |  1 |  1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
O  4 ||  1 |  1 |  1 |  1 | -1 | -1 | -1 | -1 | -1 | -1 |
M  5 ||  1 |  1 |  1 |  1 |  1 | -1 | -1 | -1 | -1 | -1 |
E  6 ||  1 |  1 |  1 |  1 |  1 |  1 | -1 | -1 | -1 | -1 |
   7 ||  1 |  1 |  1 |  1 |  1 |  1 |  1 | -1 | -1 | -1 |
   8 ||  1 |  1 |  1 |  1 |  1 |  1 |  1 |  1 | -1 | -1 |
   9 ||  1 |  1 |  1 |  1 |  1 |  1 |  1 |  1 |  1 | -1 |
```

#### Asian Handicap

```bash
bet_grid --bet-type=asian-handicap --side=home --handicap=1
bet_grid --bet-type=asian-handicap --side=away --handicap=-2.5
```

Output: Displays a grid showing outcomes based on the handicap value. The handicap shifts the results accordingly.

Example output for `--side=home --handicap=1`:

```bash
                         AWAY
     || 0 | 1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 |
-------------------------------------------------------
   0 || 1 | 0 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
   1 || 1 | 1 |  0 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
   2 || 1 | 1 |  1 |  0 | -1 | -1 | -1 | -1 | -1 | -1 |
H  3 || 1 | 1 |  1 |  1 |  0 | -1 | -1 | -1 | -1 | -1 |
O  4 || 1 | 1 |  1 |  1 |  1 |  0 | -1 | -1 | -1 | -1 |
M  5 || 1 | 1 |  1 |  1 |  1 |  1 |  0 | -1 | -1 | -1 |
E  6 || 1 | 1 |  1 |  1 |  1 |  1 |  1 |  0 | -1 | -1 |
   7 || 1 | 1 |  1 |  1 |  1 |  1 |  1 |  1 |  0 | -1 |
   8 || 1 | 1 |  1 |  1 |  1 |  1 |  1 |  1 |  1 |  0 |
   9 || 1 | 1 |  1 |  1 |  1 |  1 |  1 |  1 |  1 |  1 |
```

#### Over Under

```bash
bet_grid --bet-type=over-under --side=under --goals=4.5
bet_grid --bet-type=over-under --side=over --goals=1
```

Output: Displays a grid showing outcomes based on the over/under goal threshold.

Example output for `--side=under --goals=4.5`:

```bash
                          AWAY
     ||  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 |
---------------------------------------------------------
   0 ||  1 |  1 |  1 |  1 |  1 | -1 | -1 | -1 | -1 | -1 |
   1 ||  1 |  1 |  1 |  1 | -1 | -1 | -1 | -1 | -1 | -1 |
   2 ||  1 |  1 |  1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
H  3 ||  1 |  1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
O  4 ||  1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
M  5 || -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
E  6 || -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
   7 || -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
   8 || -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
   9 || -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 | -1 |
```

## Development

You'll (obviously) need to install [Rust](https://www.rust-lang.org/tools/install).

Other than that, just edit the code, build and that's it.

Some example commands you can use to test output while developing:

```bash
cargo run -- --bet-type=win-draw-win --side=a
cargo run -- --bet-type=asian-handicap --side=home --handicap=1
```

### Windows

You can run `$Env:RUST_BACKTRACE = "1"` to get backtrace for particularly nasty crashes

### Linux

Run `export RUST_BACKTRACE=1` before cargo run to get the full backtrace.

### Building

You can run `make build` to build the project.

After building the project, the `bet_grid` binary will be located in the `target/<platform>/release/` directory.

### Testing

There are unit tests sprinkled throughout the modules.

Run `cargo test` to run the test suite.

# License

This project is licensed under the MIT License - see the LICENSE file for details.
