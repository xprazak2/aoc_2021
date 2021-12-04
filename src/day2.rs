#[derive(Debug)]
enum Direction {
  Up(u32),
  Down(u32),
  Forward(u32)
}

fn from_input(input: &str) -> Vec<Direction> {
  input.lines().map(|line| {
    let split: Vec<&str> = line.trim().split_whitespace().collect();
    match split[0] {
      "forward" => Direction::Forward(split[1].parse::<u32>().unwrap()),
      "down" => Direction::Down(split[1].parse::<u32>().unwrap()),
      "up" => Direction::Up(split[1].parse::<u32>().unwrap()),
      _ => panic!("Unknown direction: {:?}", split[0])
    }
  }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
  let directions = from_input(input);
  let mut horizontal = 0;
  let mut depth = 0;
  for direction in directions {
    match direction {
      Direction::Up(num) => depth -= num,
      Direction::Down(num) => depth += num,
      Direction::Forward(num) => horizontal += num,
    }
  }
  depth * horizontal
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
  let directions = from_input(input);
  let mut aim = 0;
  let mut horizontal = 0;
  let mut depth = 0;
  for direction in directions {
    match direction {
      Direction::Up(num) => aim -= num,
      Direction::Down(num) => aim += num,
      Direction::Forward(num) => {
        horizontal += num;
        depth += aim * num;
      }
    }
  }
  depth * horizontal
}
