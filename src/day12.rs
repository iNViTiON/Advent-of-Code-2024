use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct Config {
    in_file: String,
}

impl Config {
    fn new(args: &mut impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let in_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing input file argument"),
        };

        Ok(Config { in_file })
    }
}

fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        std::process::exit(1);
    })
}

type Position = (u8, u8);

struct Region<'a> {
    plant_type: char,
    plots: HashSet<&'a Position>,
}

fn to_plots(raw_dataset: &str) -> HashMap<Position, char> {
    raw_dataset
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                acc.insert((row as u8, col as u8), c);
            });
            acc
        })
}

fn expand_region<'a>(
    region: &mut Region<'a>,
    plots: &'a HashMap<Position, char>,
    taken: &mut HashSet<&'a Position>,
    position: &'a Position,
) {
    if taken.contains(position)
        || !plots.contains_key(&position)
        || plots[&position] != region.plant_type
    {
        return;
    }
    taken.insert(position);
    region.plots.insert(position);
    let (row, col) = position;
    for row_offset in [row.checked_sub(1), row.checked_add(1)]
        .into_iter()
        .filter_map(|y| y)
    {
        let pos = &(row_offset, *col);
        if let Some((pos, _)) = plots.get_key_value(pos) {
            expand_region(region, plots, taken, pos);
        }
    }
    for col_offset in [col.checked_sub(1), col.checked_add(1)]
        .into_iter()
        .filter_map(|x| x)
    {
        let pos = &(*row, col_offset);
        if let Some((pos, _)) = plots.get_key_value(pos) {
            expand_region(region, plots, taken, pos);
        }
    }
}

fn to_regions(
    plots: &HashMap<Position, char>,
) -> Vec<Region> {
    let mut taken_plots: HashSet<&Position> = HashSet::new();
    let regions: Vec<Region> = plots
        .iter()
        .fold(Vec::new(), |mut acc, (position, plant_type)| {
            if taken_plots.contains(position) {
                return acc;
            }
            let mut region = Region {
                plant_type: *plant_type,
                plots: HashSet::new(),
            };
            expand_region(&mut region, &plots, &mut taken_plots, position);

            acc.push(region);
            acc
        });
    regions
}

fn count_perimeter(region: &Region) -> u8 {
    region
        .plots
        .iter()
        .map(|position| {
            let (row, col) = position;
            let row_iter = [row.wrapping_sub(1), row.wrapping_add(1)]
                .into_iter()
                .map(|row_offset| (row_offset, *col));
            let col_iter = [col.wrapping_sub(1), col.wrapping_add(1)]
                .into_iter()
                .map(|col_offset| (*row, col_offset));
            row_iter
                .chain(col_iter)
                .filter(|neighbor_position| !region.plots.contains(neighbor_position))
                .count() as u8
        })
        .sum()
}

fn price_by_perimeters(region: &Region) -> usize {
    region.plots.len() * count_perimeter(region) as usize
}

fn total_price_by_perimeters(regions: &[Region]) -> usize {
    regions.iter().map(price_by_perimeters).sum()
}

fn count_side(region: &Region) -> u8 {
    let top_perimeter: HashSet<&Position> = region
        .plots
        .iter()
        .filter(|(row, col)| !region.plots.contains(&(row.wrapping_sub(1), *col)))
        .map(|p| *p)
        .collect();
    let bottom_perimeter: HashSet<&Position> = region
        .plots
        .iter()
        .filter(|(row, col)| !region.plots.contains(&(row.wrapping_add(1), *col)))
        .map(|p| *p)
        .collect();
    let left_perimeter: HashSet<&Position> = region
        .plots
        .iter()
        .filter(|(row, col)| !region.plots.contains(&(*row, col.wrapping_sub(1))))
        .map(|p| *p)
        .collect();
    let right_perimeter: HashSet<&Position> = region
        .plots
        .iter()
        .filter(|(row, col)| !region.plots.contains(&(*row, col.wrapping_add(1))))
        .map(|p| *p)
        .collect();
    let top_side: HashMap<u8, HashSet<u8>> =
        top_perimeter
            .iter()
            .fold(HashMap::new(), |mut acc, (row, col)| {
                acc.entry(*row).or_insert(HashSet::new()).insert(*col);
                acc
            });
    let bottom_side: HashMap<u8, HashSet<u8>> =
        bottom_perimeter
            .iter()
            .fold(HashMap::new(), |mut acc, (row, col)| {
                acc.entry(*row).or_insert(HashSet::new()).insert(*col);
                acc
            });
    let left_side: HashMap<u8, HashSet<u8>> =
        left_perimeter
            .iter()
            .fold(HashMap::new(), |mut acc, (row, col)| {
                acc.entry(*col).or_insert(HashSet::new()).insert(*row);
                acc
            });
    let right_side: HashMap<u8, HashSet<u8>> =
        right_perimeter
            .iter()
            .fold(HashMap::new(), |mut acc, (row, col)| {
                acc.entry(*col).or_insert(HashSet::new()).insert(*row);
                acc
            });
    [top_side, bottom_side, left_side, right_side]
        .iter()
        .map(|side| {
            side.iter()
                .map(|(_, plot_sides)| {
                    plot_sides.iter()
                        .filter(|plot_side| !plot_sides.contains(&plot_side.wrapping_add(1)))
                        .count() as u8
                }).sum::<u8>()
        })
        .sum::<u8>()
}

fn price_by_sides(region: &Region) -> usize {
    region.plots.len() * count_side(region) as usize
}

fn total_price_by_sides(regions: &[Region]) -> usize {
    regions.iter().map(price_by_sides).sum()
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);

    let plots = to_plots(&raw_dataset);

    let regions = to_regions(&plots);

    let price_by_perimeters = total_price_by_perimeters(&regions);
    println!("Price by perimeter: {}", price_by_perimeters);

    let price_by_sides = total_price_by_sides(&regions);
    println!("Price by sides: {}", price_by_sides);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex() {
        let raw_dataset = read_input_file("input/day12_ex.txt");
        let plots = to_plots(&raw_dataset);
        let regions = to_regions(&plots);
        let price_by_perimeters = total_price_by_perimeters(&regions);
        let price_by_sides = total_price_by_sides(&regions);
        assert_eq!(price_by_perimeters, 1930);
        assert_eq!(price_by_sides, 1206);
    }

    #[test]
    fn test_actual() {
        let raw_dataset = read_input_file("input/day12.txt");
        let plots = to_plots(&raw_dataset);
        let regions = to_regions(&plots);
        let price_by_perimeters = total_price_by_perimeters(&regions);
        let price_by_sides = total_price_by_sides(&regions);
        assert_eq!(price_by_perimeters, 1477762);
        assert_eq!(price_by_sides, 923480);
    }
}
