use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum HomeAwayDrawSide {
    Home,
    Away,
    Draw
}

impl FromStr for HomeAwayDrawSide {
    type Err = ();

    fn from_str(input: &str) -> Result<HomeAwayDrawSide, ()> {
        match input {
            "home"  => Ok(HomeAwayDrawSide::Home),
            "h"  => Ok(HomeAwayDrawSide::Home),
            "away"  => Ok(HomeAwayDrawSide::Away),
            "a"  => Ok(HomeAwayDrawSide::Away),
            "draw"  => Ok(HomeAwayDrawSide::Draw),
            "d"  => Ok(HomeAwayDrawSide::Draw),
            _       => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OverUnderSide {
    Over,
    Under
}

impl FromStr for OverUnderSide {
    type Err = ();

    fn from_str(input: &str) -> Result<OverUnderSide, ()> {
        match input {
            "over"  => Ok(OverUnderSide::Over),
            "o"  => Ok(OverUnderSide::Over),
            "under"  => Ok(OverUnderSide::Under),
            "u"  => Ok(OverUnderSide::Under),
            _       => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HomeAwaySide {
    Home,
    Away
}

impl FromStr for HomeAwaySide {
    type Err = ();

    fn from_str(input: &str) -> Result<HomeAwaySide, ()> {
        match input {
            "home"  => Ok(HomeAwaySide::Home),
            "h"  => Ok(HomeAwaySide::Home),
            "away"  => Ok(HomeAwaySide::Away),
            "a"  => Ok(HomeAwaySide::Away),
            _       => Err(()),
        }
    }
}
