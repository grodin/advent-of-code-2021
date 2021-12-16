use color_eyre::eyre::{eyre, Report, Result};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParsePointError(#[from] color_eyre::eyre::Report);

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(',');

        fn parse_coordinate(o: Option<&str>) -> Result<usize, ParsePointError> {
            o.ok_or(ParsePointError(eyre!("No x value")))?
                .parse::<usize>()
                .map_err(|e| ParsePointError(e.into()))
        }
        let x = parse_coordinate(splits.next())?;
        let y = parse_coordinate(splits.next())?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal,
    Diagonal,
}

#[derive(Debug)]
pub struct Line {
    pub initial_point: Point,
    pub terminal_point: Point,
}

impl Line {
    pub fn orientation(&self) -> Orientation {
        if self.initial_point.x == self.terminal_point.x {
            return Orientation::Vertical;
        }
        if self.initial_point.y == self.terminal_point.y {
            return Orientation::Horizontal;
        }
        Orientation::Diagonal
    }

    pub fn is_vertical(&self) -> bool {
        self.orientation() == Orientation::Vertical
    }

    pub fn is_horizontal(&self) -> bool {
        self.orientation() == Orientation::Horizontal
    }

    pub fn points(&self) -> Result<Vec<Point>> {
        fn range_inclusive(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
            if end >= start {
                Box::new(start..=end)
            } else {
                Box::new((end..=start).rev())
            }
        }
        match self.orientation() {
            Orientation::Vertical => {
                Ok(range_inclusive(self.initial_point.y, self.terminal_point.y)
                    .map(|y| Point {
                        x: self.initial_point.x,
                        y,
                    })
                    .collect())
            }
            Orientation::Horizontal => {
                Ok(range_inclusive(self.initial_point.x, self.terminal_point.x)
                    .map(|x| Point {
                        x,
                        y: self.initial_point.y,
                    })
                    .collect())
            }
            Orientation::Diagonal => {
                Ok(range_inclusive(self.initial_point.x, self.terminal_point.x)
                    .zip(range_inclusive(self.initial_point.y, self.terminal_point.y))
                    .map(|(x, y)| Point { x, y })
                    .collect())
            }
        }
    }
}

impl FromStr for Line {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coord_pairs = s
            .split(" -> ")
            .map(|pair| pair.parse::<Point>())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();
        let (initial, terminal) = (
            coord_pairs
                .next()
                .ok_or_else(|| eyre!("No initial point"))?,
            coord_pairs
                .next()
                .ok_or_else(|| eyre!("No terminal point"))?,
        );

        Ok(Line {
            initial_point: initial,
            terminal_point: terminal,
        })
    }
}
