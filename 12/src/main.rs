#[derive(PartialEq, Debug)]
struct PresentShape {
    id: i64,
    spaces: Vec<Vec<bool>>,
}

fn to_present_shape(paragraph: &str) -> PresentShape {
    let lines: Vec<_> = paragraph.lines().collect();
    let id = lines[0][0..1].parse().unwrap();
    let spaces = paragraph
        .lines()
        .skip(1)
        .map(|line| line.as_bytes().iter().map(|b| *b == b'#').collect())
        .collect();

    PresentShape { id, spaces }
}

#[derive(Debug, PartialEq)]
struct PresentsStorage {
    width: i64,
    height: i64,
    required_presents: Vec<usize>,
}

fn to_present_storage(line: &str) -> PresentsStorage {
    let segments: Vec<_> = line.split(": ").collect();
    let shape: Vec<_> =  segments[0].split("x").map(|s| s.parse().unwrap() ).collect();
    let required_presents = segments[1].split(" ").map(|s| s.parse().unwrap() ).collect();

    PresentsStorage { width: shape[0], height: shape[1], required_presents }
}
fn main() {
    let start = std::time::Instant::now();
    let input = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let presents: Vec<_> = input.split("\n\n").take(6).map(to_present_shape).collect();
    let storages: Vec<_> = input.split("\n\n").last().map(|p| p.lines().map(to_present_storage).collect()).unwrap();

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::{PresentShape, PresentsStorage, to_present_shape, to_present_storage};

    #[test]
    fn to_present_shape__given_shape__builds_data_structure() {
        let input = "1:\n###\n##.\n.##";
        let result = to_present_shape(input);
        let expected = PresentShape{
            id: 1,
            spaces: vec![vec![true, true, true], vec![true, true, false], vec![false, true, true], ]
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn to_present_storage__given_input__builds_data_structure() {
        let input = "10x5: 1 3 5 4 2 0";
        let result = to_present_storage(input);
        let expected = PresentsStorage{
            width: 10,
            height: 5,
            required_presents: vec![1,3,5,4,2,0],
        };

        assert_eq!(result,expected);
    }

}