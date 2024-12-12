use std::collections::HashSet;

use crate::{plot::Plot, region::Region};

#[derive(Debug)]
pub struct Farm {
    plots: Vec<Plot>,
    width: i32,
    height: i32,
}

impl Farm {
    pub fn new(input: &str) -> Self {
        let mut plots = vec![];

        let mut width = 0;

        let mut x = 0;
        let mut y = 0;

        for c in input.chars() {
            if c == '\n' {
                if width == 0 {
                    width = x;
                }

                x = 0;
                y += 1;

                continue;
            }

            plots.push(Plot::new(x, y, c));
            x += 1;
        }

        Self {
            plots,
            width: if width == 0 { x } else { width },
            height: y + 1,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<Plot> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }

        let index = y * self.width + x;
        self.plots.get(index as usize).copied()
    }

    pub fn get_regions(&self) -> Vec<Region> {
        let mut visited = HashSet::new();
        let mut regions: Vec<Region> = vec![];

        for plot in &self.plots {
            if visited.contains(&(plot.x, plot.y)) {
                continue;
            }

            let mut region = Region::new(plot.plant);

            self.populate_region(plot, &mut region, &mut visited);
            regions.push(region);
        }

        regions
    }

    fn populate_region(&self, plot: &Plot, region: &mut Region, visited: &mut HashSet<(i32, i32)>) {
        visited.insert((plot.x, plot.y));
        region.add(plot.x, plot.y);

        for (x, y) in plot.neighbours() {
            let Some(neighbor) = self.get(x, y) else {
                region.increment_parimeter();
                continue;
            };

            if neighbor.plant != region.plant {
                region.increment_parimeter();
                continue;
            }

            if !region.has(x, y) {
                self.populate_region(&neighbor, region, visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = ["AAAA", "BBCD", "BBCC", "EEEC"].join("\n");
        let farm = Farm::new(&input);
        let regions = farm.get_regions();
        assert_eq!(regions.len(), 5);
    }

    #[test]
    fn test_2() {
        let input = ["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"].join("\n");
        let farm = Farm::new(&input);
        let regions = farm.get_regions();
        assert_eq!(regions.len(), 5);
    }
}
