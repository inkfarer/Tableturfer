use std::num::TryFromIntError;
use serde::{Deserialize, Serialize};

pub type IPosition = (isize, isize);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct INamedPosition {
    pub x: isize,
    pub y: isize,
}

impl INamedPosition {
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl From<IPosition> for INamedPosition {
    fn from(pos: (isize, isize)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
        }
    }
}

pub type UPosition = (usize, usize);

impl From<UNamedPosition> for UPosition {
    fn from(pos: UNamedPosition) -> Self {
        (pos.x, pos.y)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UNamedPosition {
    pub x: usize,
    pub y: usize,
}

impl UNamedPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl TryFrom<INamedPosition> for UNamedPosition {
    type Error = TryFromIntError;

    fn try_from(value: INamedPosition) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}

impl From<UPosition> for UNamedPosition {
    fn from(pos: (usize, usize)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
        }
    }
}
