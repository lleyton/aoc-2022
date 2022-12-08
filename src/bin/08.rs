use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn get_all(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let height = map.len();
    let width = map[0].len();

    (0..height)
        .flat_map(|height| {
            (0..width)
                .map(|width| (height, width))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

// pub fn get_adjacent_visible(map: &Vec<Vec<u32>>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
//     let mut to_check = vec![];
//     let (height, width) = *pos;

//     if height > 0 {
//         to_check.push((height - 1, width));
//     }

//     if height < map.len() - 1 {
//         to_check.push((height + 1, width));
//     }

//     if width > 0 {
//         to_check.push((height, width - 1));
//     }

//     if width < map[0].len() - 1 {
//         to_check.push((height, width + 1));
//     }

//     to_check
//         .iter()
//         .filter(|check| map[check.0][check.1] > map[height][width])
//         .map(|c| *c)
//         .collect()
// }

pub fn visible_from_edge(map: &Vec<Vec<u32>>, pos: &(usize, usize)) -> bool {
    let (height, width) = *pos;

    if height == 0 || width == 0 || height == (map.len() - 1) || width == (map[0].len() - 1) {
        return true;
    }

    for h in (0..height).rev() {
        if map[h][width] >= map[height][width] {
            break;
        }

        if h == 0 {
            return true;
        }
    }

    for h in (height + 1)..(map.len()) {
        if map[h][width] >= map[height][width] {
            break;
        }

        if h == (map.len() - 1) {
            return true;
        }
    }

    for w in (0..width).rev() {
        if map[height][w] >= map[height][width] {
            break;
        }

        if w == 0 {
            return true;
        }
    }

    for w in (width + 1)..(map[0].len()) {
        if map[height][w] >= map[height][width] {
            break;
        }

        if w == (map[0].len() - 1) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let all = get_all(&map);
    let points = all
        .iter()
        .filter(|point| visible_from_edge(&map, point))
        .collect::<Vec<&(usize, usize)>>();

    Some(points.len() as u32)
}

pub fn score(map: &Vec<Vec<u32>>, pos: &(usize, usize)) -> u32 {
    let mut scores: Vec<usize> = vec![];
    let (height, width) = *pos;

    if height == 0 || width == 0 || height == (map.len() - 1) || width == (map[0].len() - 1) {
        return 1;
    }


    for h in (0..height).rev() {
        if map[h][width] >= map[height][width] {
            scores.push(height - h);
            break;
        }

        if h == 0 {
            scores.push(height - h);
        }
    }

    for h in (height + 1)..(map.len()) {
        if map[h][width] >= map[height][width] {
            scores.push(h - height);
            break;
        }

        if h == (map.len() - 1) {
            scores.push(h - height);
        }
    }

    for w in (0..width).rev() {
        if map[height][w] >= map[height][width] {
            scores.push(width - w);
            break;
        }

        if w == 0 {
            scores.push(width - w);
        }
    }

    for w in (width + 1)..(map[0].len()) {
        if map[height][w] >= map[height][width] {
            scores.push(w - width);
            break;
        }

        if w == (map.len() - 1) {
            scores.push(w - width);
        }
    }

    if scores.len() == 0 {
        return 1;
    }

    scores.iter().map(|x| *x as u32).reduce(|a, b| a * b).unwrap()
}


pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let all = get_all(&map);
    let mut points = all
        .iter()
        .map(|point| score(&map, point))
        .collect::<Vec<u32>>();

    points.sort();

    Some(*points.last().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
