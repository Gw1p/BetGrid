use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum HomeAwayDrawSide {
    Home,
    Away,
    Draw,
}

impl FromStr for HomeAwayDrawSide {
    type Err = ();

    fn from_str(input: &str) -> Result<HomeAwayDrawSide, ()> {
        match input {
            "home" => Ok(HomeAwayDrawSide::Home),
            "h" => Ok(HomeAwayDrawSide::Home),
            "away" => Ok(HomeAwayDrawSide::Away),
            "a" => Ok(HomeAwayDrawSide::Away),
            "draw" => Ok(HomeAwayDrawSide::Draw),
            "d" => Ok(HomeAwayDrawSide::Draw),
            _ => Err(()),
        }
    }
}

impl fmt::Display for HomeAwayDrawSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HomeAwayDrawSide::Home => write!(f, "Home"),
            HomeAwayDrawSide::Away => write!(f, "Away"),
            HomeAwayDrawSide::Draw => write!(f, "Draw"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OverUnderSide {
    Over,
    Under,
}

impl FromStr for OverUnderSide {
    type Err = ();

    fn from_str(input: &str) -> Result<OverUnderSide, ()> {
        match input {
            "over" => Ok(OverUnderSide::Over),
            "o" => Ok(OverUnderSide::Over),
            "under" => Ok(OverUnderSide::Under),
            "u" => Ok(OverUnderSide::Under),
            _ => Err(()),
        }
    }
}

impl fmt::Display for OverUnderSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OverUnderSide::Over => write!(f, "Over"),
            OverUnderSide::Under => write!(f, "Under"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum HomeAwaySide {
    Home,
    Away,
}

impl FromStr for HomeAwaySide {
    type Err = ();

    fn from_str(input: &str) -> Result<HomeAwaySide, ()> {
        match input {
            "home" => Ok(HomeAwaySide::Home),
            "h" => Ok(HomeAwaySide::Home),
            "away" => Ok(HomeAwaySide::Away),
            "a" => Ok(HomeAwaySide::Away),
            _ => Err(()),
        }
    }
}

impl fmt::Display for HomeAwaySide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HomeAwaySide::Home => write!(f, "Home"),
            HomeAwaySide::Away => write!(f, "Away"),
        }
    }
}
