use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

const ORIGIN: Point = Point::new(0, 0);
const UP: Point = Point::new(0, 1);
const DOWN: Point = Point::new(0, -1);
const LEFT: Point = Point::new(-1, 0);
const RIGHT: Point = Point::new(1, 0);
const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    const fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn manhattan(self, other: Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

impl From<u8> for Point {
    fn from(value: u8) -> Self {
        match value {
            b'^' | b'U' => UP,
            b'v' | b'D' => DOWN,
            b'<' | b'L' => LEFT,
            b'>' | b'R' => RIGHT,
            _ => unreachable!(),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

struct Pad {
    start: Point,
    empty_space: Point,
    lookup: [Point; 128],
}

const NUMERIC_PAD: Pad = {
    let start = Point::new(2, 0);
    let empty_space = ORIGIN;
    let mut lookup = [ORIGIN; 128];

    lookup[b'0' as usize] = Point::new(1, 0);
    lookup[b'A' as usize] = Point::new(2, 0);
    lookup[b'1' as usize] = Point::new(0, 1);
    lookup[b'2' as usize] = Point::new(1, 1);
    lookup[b'3' as usize] = Point::new(2, 1);
    lookup[b'4' as usize] = Point::new(0, 2);
    lookup[b'5' as usize] = Point::new(1, 2);
    lookup[b'6' as usize] = Point::new(2, 2);
    lookup[b'7' as usize] = Point::new(0, 3);
    lookup[b'8' as usize] = Point::new(1, 3);
    lookup[b'9' as usize] = Point::new(2, 3);

    Pad {
        start,
        empty_space,
        lookup,
    }
};

const DIRECTIONAL_PAD: Pad = {
    let start = Point::new(2, 1);
    let empty_space = Point::new(0, 1);
    let mut lookup = [ORIGIN; 128];

    lookup[b'<' as usize] = ORIGIN;
    lookup[b'v' as usize] = Point::new(1, 0);
    lookup[b'>' as usize] = Point::new(2, 0);
    lookup[b'^' as usize] = Point::new(1, 1);
    lookup[b'A' as usize] = Point::new(2, 1);

    Pad {
        start,
        empty_space,
        lookup,
    }
};

pub fn part_one<const NUM_PADS: usize>(input: &str) -> usize {
    let mut previously_seen: HashMap<(Vec<char>, usize), usize> = HashMap::new(); // (start, end) --> distance

    input
        .lines()
        .map(|line| {
            let (numeric_part, remaining) = line.split_at(line.len() - 1);
            let numeric: usize = numeric_part
                .trim_start_matches('0')
                .parse::<usize>()
                .ok()
                .unwrap();
            (line.chars().collect::<Vec<char>>(), numeric)
        })
        .map(|(code, numeric)| dfs(&mut previously_seen, &code.clone(), 0, NUM_PADS) * numeric)
        .sum()
}

fn dfs(
    previously_seen: &mut HashMap<(Vec<char>, usize), usize>,
    slice: &Vec<char>,
    depth: usize,
    num_pads: usize,
) -> usize {
    if depth == num_pads {
        return slice.len();
    }

    let key = (slice.clone(), depth);
    if let Some(&previous) = previously_seen.get(&key) {
        return previous;
    }

    let pad: Pad = {
        if depth == 0 {
            NUMERIC_PAD
        } else {
            DIRECTIONAL_PAD
        }
    };

    let mut shortest = usize::MAX;

    for sequence in combinations(slice, &pad) {
        let mut presses = 0;

        for chunk in sequence.split_inclusive(|&button| button == 'A') {
            presses += dfs(previously_seen, &Vec::from(chunk), depth + 1, num_pads);
        }

        shortest = shortest.min(presses);
    }

    previously_seen.insert(key, shortest);
    shortest
}

fn combinations(current: &Vec<char>, pad: &Pad) -> Vec<Vec<char>> {
    let mut next: Vec<Vec<char>> = Vec::new();
    pad_dfs(&mut next, &mut Vec::new(), pad, current, 0, pad.start);
    next
}

fn pad_dfs(
    combinations: &mut Vec<Vec<char>>,
    path: &mut Vec<char>,
    pad: &Pad,
    sequence: &Vec<char>,
    depth: usize,
    from: Point,
) {
    if depth == sequence.len() {
        combinations.push(path.clone());
        return;
    }

    if from == pad.empty_space {
        return;
    }

    let to = pad.lookup[sequence[depth] as usize];

    if from == to {
        path.push('A');
        pad_dfs(combinations, path, pad, sequence, depth + 1, from);
        path.pop();
    } else {
        let mut step = |next: char, direction: Point| {
            path.push(next);
            pad_dfs(combinations, path, pad, sequence, depth, from + direction);
            path.pop();
        };

        if to.x < from.x {
            step('<', LEFT);
        }
        if to.x > from.x {
            step('>', RIGHT);
        }
        if to.y > from.y {
            step('^', UP);
        }
        if to.y < from.y {
            step('v', DOWN);
        }
    }
}
