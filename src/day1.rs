#[aoc_generator(day1)]
pub fn part_1_generator(input: &str) -> Vec<u32> {
  input.lines().map(|line| {
    line.trim().parse::<u32>().unwrap()
  }).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
  count_increases(input)
}

fn count_increases(input: &[u32]) -> u32 {
  let mut total = 0;
  input.iter().reduce(|a, b| {
    if b > a {
      total += 1;
    }
    b
  });
  total
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
  let mut memo = 0;
  let mut nums = vec![];
  for (idx, item) in input.iter().enumerate() {
    memo += item;
    if idx > 2 {
      memo -= input[idx - 3];
    }
    if idx > 1 {
      nums.push(memo);
    }
  }
  count_increases(&nums)
}