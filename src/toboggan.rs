use std::str::FromStr;
use anyhow::Error;

pub struct Slope {
    width: usize,
    map: Vec<u8>,
}

impl Slope {
    pub fn width(&self) -> isize { self.width as isize }
    pub fn height(&self) -> isize { (self.map.len() / self.width) as isize }

    fn map_x(&self, x: isize) -> usize {
        let w = self.width();
        let x = x % w;
        if x < 0 {
            (x + w) as usize
        } else {
            x as usize
        }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<u8> {
        if y >= self.height() {
            return None;
        }

        let x = self.map_x(x);
        let y = y as usize;
        Some(self.map[y * self.width + x])
    }
}

impl FromStr for Slope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty());
        let first_line = lines.next()
            .ok_or_else(|| Error::msg("slope must have at least one line"))?;
        let width = first_line.len();
        let mut map = Vec::with_capacity(s.len());
        map.extend(first_line.bytes());

        for line in lines {
            if line.len() != width {
                Err(Error::msg("all lines must be the same width"))?;
            }

            map.extend(line.bytes());
        }

        Ok(Slope{
            width,
            map,
        })
    }
}