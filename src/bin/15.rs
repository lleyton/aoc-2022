use itertools::Itertools;

pub type Coordinate = (i64, i64);
pub type Reading = (Coordinate, Coordinate);

pub fn parse_input(input: &str) -> Vec<Reading> {
    input
        .lines()
        .map(|line| {
            let (sensor_part, beacon_part) = line.split_once(": ").unwrap();
            let (sensor_x_part, sensor_y_part) = sensor_part.split_once(", ").unwrap();
            let (beacon_x_part, beacon_y_part) = beacon_part.split_once(", ").unwrap();

            let sensor_x = sensor_x_part
                .strip_prefix("Sensor at x=")
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let sensor_y = sensor_y_part
                .strip_prefix("y=")
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let beacon_x = beacon_x_part
                .strip_prefix("closest beacon is at x=")
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let beacon_y = beacon_y_part
                .strip_prefix("y=")
                .unwrap()
                .parse::<i64>()
                .unwrap();

            ((sensor_x, sensor_y), (beacon_x, beacon_y))
        })
        .collect_vec()
}

pub fn filter_coordinates(readings: &Vec<Reading>, y_filter: i64) -> Vec<Coordinate> {
    let beacons = readings.iter().map(|reading| reading.1).collect_vec();

    readings
        .iter()
        .flat_map(|reading| {
            let distance =
                (reading.0 .0 - reading.1 .0).abs() + (reading.0 .1 - reading.1 .1).abs();

            let min_x = reading.0 .0 - distance;
            let max_x = reading.0 .0 + distance;
            let min_y = reading.0 .1 - distance;
            let max_y = reading.0 .1 + distance;

            if (y_filter < min_y) || (y_filter > max_y) {
                return Vec::new();
            }

            let min_x = if min_x < max_x { min_x } else { max_x };
            let max_x = if min_x > max_x { min_x } else { max_x };

            (min_x..=max_x)
                .filter(|x| (x - reading.0 .0).abs() + (y_filter - reading.0 .1).abs() <= distance)
                .map(|x| (x, y_filter))
                .collect_vec()
        })
        .unique()
        .filter(|coordinate| !beacons.contains(coordinate))
        .collect()
}

pub fn find_beacon(readings: &Vec<Reading>, max_search_x: i64, max_search_y: i64) -> Coordinate {
    let circles = readings
        .iter()
        .map(|reading| {
            let distance =
                (reading.0 .0 - reading.1 .0).abs() + (reading.0 .1 - reading.1 .1).abs();

            (reading.0 .0, reading.0 .1, distance)
        })
        .collect_vec();

    for x in 0..=max_search_x {
        let mut y_iter = 0..=max_search_y;

        while let Some(y) = y_iter.next() {
            if let Some(circle) = circles.iter().find(|circle| (x - circle.0).abs() + (y - circle.1).abs() <= circle.2) {
                y_iter = (circle.1 + (circle.2 - (circle.0 - x).abs()) + 1)..=max_search_y;
            } else {
                return (x, y);
            }
        }
    }

    (0, 0)
}

pub fn part_one(input: &str) -> Option<i64> {
    let readings = parse_input(input);
    let coordinates = filter_coordinates(&readings, 2000000);

    Some(coordinates.len() as i64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let readings = parse_input(input);
    let coordinate = find_beacon(&readings, 4000000, 4000000);

    println!("{}", (coordinate.0 * 4000000 + coordinate.1));

    Some(coordinate.0 * 4000000 + coordinate.1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
