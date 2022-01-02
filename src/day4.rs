use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
struct Point {
  value: usize,
  marked: bool,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Copy, Clone)]
struct Coord {
  col: usize,
  row: usize,
}

#[derive(Debug, Clone)]
struct Table {
  table: Vec<Vec<Point>>,
  values: HashMap<usize, Coord>,
}

#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Table {
  fn new(size: usize) -> Self {
    let mut rows = Vec::with_capacity(size);
    for _ in 0..size  {
      rows.push(Vec::with_capacity(size));
    }
    Self{table: rows, values: HashMap::new()}
  }

  pub fn append_to_row(&mut self, value: usize, row: usize) {
    let col = self.table[row].len();
    let coord = Coord{col, row};
    let point = Point{value: value, marked: false};
    self.table[row].push(point);
    self.values.insert(value, coord);
  }

  pub fn get_by_coord<'a>(table: &'a Self, coord: &'a Coord) -> &'a Point {
    &table.table[coord.row][coord.col]
  }

  pub fn mark(&mut self, value: usize) {
    if let Some(coord) = self.values.get(&value) {
      self.table[coord.row][coord.col].marked = true;
    }
  }

  pub fn check_victory(table: &Self, value: usize) -> bool {
    if let Some(coord) = table.values.get(&value) {
      let win = (Table::check_direction(table, *coord, Direction::Left) && Table::check_direction(table, *coord, Direction::Right)) || (Table::check_direction(table, *coord, Direction::Up) && Table::check_direction(table, *coord, Direction::Down));
      return win;
    }
    false
  }

  fn check_direction(table: &Self, coord: Coord, direction: Direction) -> bool {
    if !Table::in_table(table, coord) {
      return true
    }

    if !Table::get_by_coord(table, &coord).marked {
      return false
    }
    match direction {
      Direction::Up => Table::check_direction(table, Coord{col: coord.col, row: coord.row - 1}, Direction::Up),
      Direction::Down => Table::check_direction(table, Coord{col: coord.col, row: coord.row + 1}, Direction::Down),
      Direction::Left => Table::check_direction(table, Coord{col: coord.col - 1, row: coord.row}, Direction::Left),
      Direction::Right => Table::check_direction(table, Coord{col: coord.col + 1, row: coord.row}, Direction::Right),
    }
  }

  fn in_table(table: &Self, coord: Coord) -> bool {
    if coord.row >= table.table.len() || coord.col >= table.table[0].len() {
      return false
    }

    true
  }

  pub fn unmarked_score(&self) -> usize {
    let mut score = 0;
      for row in &self.table {
        for point in row {
          if !point.marked {
            score += point.value;
          }
        }
      }
    score
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let space = if self.value < 10 {
      " "
    } else {
      ""
    };
    let val = if self.marked {
      format!("{}{}x", space, self.value)
    } else {
      format!("{}{} ", space, self.value)
    };
    write!(f, "{}", val)
  }
}

impl fmt::Display for Table {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut val: String = "".into();
    for row in &self.table {
      let row_str: Vec<String> = row.iter().map(|point| format!("{}", point)).collect();
      val.push_str(&row_str.join(" ".into()));
      val.push_str("\n".into());
    }
    write!(f, "{}", val)
  }
}

#[derive(Debug)]
struct Bingo {
  pub tables: Vec<Table>,
  pub inputs: Vec<usize>,
  pub turn: usize,
  pub winner_indexes: Vec<usize>,
  pub last_winner_idx: Option<usize>
}

impl Bingo {
  pub fn new() -> Self {
    Self{ tables: vec![], inputs: vec![], turn: 0, winner_indexes: vec![], last_winner_idx: None }
  }

  pub fn winners(&self) -> Vec<Table> {
    let mut tables: Vec<_> = vec![];
    for idx in self.winner_indexes.iter() {
      let clone = self.tables[*idx].clone();
      tables.push(clone);
    }
    tables
  }

  pub fn finished(&self) -> bool {
    self.turn >= self.inputs.len()
  }

  pub fn last_drawn(&self) -> usize {
    self.inputs[self.turn - 1]
  }

  pub fn any_winners(&mut self) -> bool {
    self.winner_indexes.len() > 0
  }

  pub fn all_winners(&self) -> bool {
    self.winner_indexes.len() == self.tables.len()
  }

  pub fn one_remaining(&self) -> bool {
    self.tables.len() == self.winner_indexes.len() + 1
  }

  pub fn remember_last(&mut self) {
    let mut sorted = self.winner_indexes.to_vec();
    sorted.sort();
    for (idx, item) in sorted.iter().enumerate() {
      if idx != *item {
        self.last_winner_idx = Some(idx);
        return;
      }
    }
  }

  pub fn next_turn(&mut self) {
    if self.finished() {
      return;
    }
    let current_number = self.inputs[self.turn];
    for (idx, table) in self.tables.iter_mut().enumerate() {
      table.mark(current_number);
      if Table::check_victory(&table, current_number) {
        if !self.winner_indexes.contains(&idx) {
          self.winner_indexes.push(idx);
        }
      }
    }
    if self.one_remaining() {
      self.remember_last();
    }
    self.turn += 1;
  }

  pub fn last_score(&self) -> usize {
    let val = match self.last_winner_idx {
      Some(val) => self.tables[val].unmarked_score(),
      None => self.find_last_score(),
    };
    val * self.last_drawn()
  }

  fn find_last_score(&self) -> usize {
    let mut all = vec![0; self.tables.len()];
    all = all.into_iter().enumerate().map(|(idx, _num)| idx).collect();
    let diff: Vec<_> = all.into_iter().filter(|item| !self.winner_indexes.contains(item)).collect();
    diff.into_iter().fold(0, |memo, idx| {
      let score = self.tables[idx].unmarked_score();
      if score > memo {
        score
      } else {
        memo
      }
    })
  }

  pub fn winning_score(&self) -> usize {
    // let mut score = 0;
    // for table in self.winners() {
    //   let table_score = table.unmarked_score() * self.last_drawn();
    //   if table_score > score {
    //     score = table_score;
    //   }
    // }
    // score
    self.winners().into_iter().fold(0, |memo, table| {
      let table_score = table.unmarked_score() * self.last_drawn();
      if table_score > memo {
        table_score
      } else {
        memo
      }
    })
  }
}

fn parse_tables(table_size: usize, mut lines: std::str::Lines) -> Bingo {
  let mut game = Bingo::new();
  let mut table = Table::new(table_size);
  let mut row = 0;
  while let Some(line) = lines.next() {
    if line.trim().is_empty() {
      continue;
    }
    for num in line.trim().split_whitespace().map(|num| num.parse::<usize>().unwrap()) {
      table.append_to_row(num, row);
    }
    row += 1;
    if row == table_size {
      game.tables.push(table);
      table = Table::new(table_size);
      row = 0;
    }
  }
  game
}

fn initialize(input: &str) -> Bingo {
  let mut lines = input.lines();
  let mut input_nums = vec![];
  if let Some(nums) = lines.next() {
    input_nums = nums.split(",").map(|num| num.parse::<usize>().unwrap()).collect();
  }
  lines.next();
  let mut game = parse_tables(5, lines);
  game.inputs = input_nums;
  game
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
  let mut game = initialize(input);
  loop {
    game.next_turn();

    if game.any_winners() || game.finished() {
      break;
    }
  }

  game.winning_score()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
  let mut game = initialize(input);
  loop {
    game.next_turn();

    if game.all_winners() || game.finished() {
      break;
    }
  }

  game.last_score()
}
