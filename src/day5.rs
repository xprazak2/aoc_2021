use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
struct Coord {
  x: usize,
  y: usize,
}

#[derive(Debug)]
struct Line {
  start: Coord,
  end: Coord
}

impl Line {
  fn new(start: Coord, end: Coord) -> Self {
    Self{ start, end }
  }

  fn vertical(&self) -> bool {
    self.start.x == self.end.x
  }

  fn horizontal(&self) -> bool {
    self.start.y == self.end.y
  }

  fn to_coords(&self) -> Vec<Coord> {
    if self.horizontal() {
      return self.to_coords_horizontal();
    }

    if self.vertical() {
      return self.to_coords_vertical();
    }
    vec![]
  }

  fn to_coords_horizontal(&self) -> Vec<Coord> {
    if self.start.x > self.end.x {
      self.walk_x(self.end.x, self.start.x, self.start.y)
    } else {
      self.walk_x(self.start.x, self.end.x, self.start.y)
    }
  }

  fn walk_x(&self, x_start: usize, x_end: usize, y_value: usize) -> Vec<Coord> {
    (x_start..=x_end).map(|x_value| Coord{x: x_value, y: y_value}).collect()
  }

  fn to_coords_vertical(&self) -> Vec<Coord> {
    if self.start.y > self.end.y {
      self.walk_y(self.end.y, self.start.y, self.start.x)
    } else {
      self.walk_y(self.start.y, self.end.y, self.start.x)
    }
  }

  fn walk_y(&self, y_start: usize, y_end: usize, x_value: usize) -> Vec<Coord> {
    (y_start..=y_end).map(|y_value| Coord{x: x_value, y: y_value}).collect()
  }
}

fn from_input(input: &str) -> Vec<Line> {
  input.lines().map(|line| {
    let coords: Vec<_> = line.trim().split(" -> ").collect();
    let start = parse_coord(coords[0]);
    let end = parse_coord(coords[1]);
    Line::new(start, end)
  }).collect()
}

fn parse_coord(coord_str: &str) -> Coord {
  let ary: Vec<_> = coord_str.split(",").map(|item| item.parse::<usize>().unwrap()).collect();
  Coord{x: ary[0], y: ary[1]}
}

#[aoc(day5, part1)]
pub fn part2(input: &str) -> usize {
  let lines = from_input(input);

  let mut map = HashMap::new();

  for line in lines {
    let coords = line.to_coords();
    for coord in coords {
      map.entry(coord).and_modify(|item| { *item += 1 }).or_insert(1);
    }
  }

  let res = map.values().fold(0, |memo, item| {
    if item > &1 { memo + 1 } else { memo }
  });

  res
}
