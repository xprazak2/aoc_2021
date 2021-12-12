fn from_input(input: &str) -> Vec<String> {
  input.lines().map(|line| {
    line.trim().split_whitespace().collect()
  }).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
  // let fake_input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
  let lines = from_input(input);

  let line_lenght = lines[0].chars().collect::<String>().len();
  let lines_count = lines.len();
  let mut result_bits = vec![0_usize; line_lenght];
  for line in lines {
    let split: Vec<_> = line.chars().collect();
    for (idx, item) in split.iter().enumerate() {
      match item.to_digit(10) {
        Some(1) => result_bits[idx] += 1,
        _ => continue
      }
    }
  }
  let gamma = calculate_gamma(result_bits, lines_count);
  let epsilon = calculate_epsilon(gamma, line_lenght);
  gamma * epsilon
}

fn lines_stats(line_lenght: usize, lines: &Vec<String>) -> Vec<usize> {
  let mut result_bits = vec![0_usize; line_lenght];
  for line in lines {
    let split: Vec<_> = line.chars().collect();
    for (idx, item) in split.iter().enumerate() {
      match item.to_digit(10) {
        Some(1) => result_bits[idx] += 1,
        _ => continue
      }
    }
  }
  return result_bits;
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
  let fake_input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
  let lines = from_input(fake_input);
  let line_lenght = lines[0].chars().collect::<String>().len();
  let lines_count = lines.len();
  let result_bits = lines_stats(line_lenght, &lines);
  let oxygen = calculate_oxygen(&lines, line_lenght);
  let co = calculate_co()
  0
}

fn calculate_co(arg: Type) -> RetType {
  unimplemented!();
}

fn calculate_oxygen(lines: &Vec<String>, line_lenght: usize) -> u32 {
  let mut result_lines = lines.clone();
  for idx in 0..line_lenght {
    let result_bits = lines_stats(line_lenght, &result_lines);
    println!("result bits: {:?}", result_bits);

    let half = if result_lines.len() % 2 == 1 {
      (result_lines.len() + 1) / 2
    } else {
      result_lines.len() / 2
    };

    result_lines = result_lines.into_iter().filter(|line| {
      let split: Vec<_> = line.chars().collect();
      (result_bits[idx] >= half && split[idx] == '1') || (result_bits[idx] < half && split[idx] == '0')
    }).collect();
    println!("{:?}", result_lines);
    if result_lines.len() == 1 {
      break;
    }
  }
  println!("Result lines {:?}", result_lines);
  let ox = isize::from_str_radix(&result_lines[0], 2).unwrap();
  println!("Oxygen: {:?}, {:08b}", ox, ox);
  0
}

fn calculate_gamma(result_bits: Vec<usize>, lines_count: usize) -> u32 {
  let mut gamma = 0;
  println!("Result bits: {:?}", result_bits);
  for (idx, one_count) in result_bits.iter().rev().enumerate() {
    if one_count > &(lines_count / 2) {
      let mask = 2_u32.pow(idx as u32);
      gamma = gamma | mask;
    }
  }
  println!("Gamma: {:?}, {:08b}", gamma, gamma);
  gamma
}

fn calculate_epsilon(gamma: u32, bits_count: usize) -> u32 {
  let mut mask = 0;
  let mut i: usize = 0;
  while i < bits_count {
    mask = mask | 2_u32.pow(i as u32);
    i += 1;
  }
  let epsilon = gamma ^ mask;
  println!("Epsilon mask: {:08b}", mask);
  println!("Epsilon: {:?}, {:08b}", epsilon, epsilon);
  epsilon
}
