use std::io::{self, Write};
use rayon::prelude::*;

const TIME_LIMIT_P1: usize = 100; // seconds elapsed
const TIME_LIMIT_P2: usize = 100_000; // seconds elapsed

#[derive(Debug)]
struct Robot {
    x_pos: usize,
    y_pos: usize,

    x_vel: isize,
    y_vel: isize,
}

impl Robot {
    fn new(x: usize, y: usize, vx: isize, vy: isize) -> Self {
        Robot {
            x_pos: x,
            y_pos: y,

            x_vel: vx,
            y_vel: vy
        }
    }

    fn update_position(&mut self, num_x_tiles: usize, num_y_tiles: usize) {
        let next_x: isize = self.x_pos as isize + self.x_vel;
        if next_x < 0 {
            self.x_pos = (num_x_tiles as isize + (next_x % num_x_tiles as isize)) as usize;
        } else {
            self.x_pos = (next_x % num_x_tiles as isize) as usize;
        }

        let next_y: isize = self.y_pos as isize + self.y_vel;
        if next_y < 0 {
            self.y_pos = (num_y_tiles as isize + (next_y % num_y_tiles as isize)) as usize;
        } else {
            self.y_pos = (next_y % num_y_tiles as isize) as usize;
        }
    }
}

pub fn part_one(input: &str, num_x_tiles: usize, num_y_tiles: usize) -> usize {

    let mut time_elapsed: usize = 0;

    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line
                .split(&['=', ',', ' '])
                .collect::<Vec<&str>>();

            Robot::new(
                split_line[1].parse::<usize>().ok().unwrap(),
                split_line[2].parse::<usize>().ok().unwrap(),
                split_line[4].parse::<isize>().ok().unwrap(),
                split_line[5].parse::<isize>().ok().unwrap()
            )
        })
        .collect::<Vec<Robot>>();

    while time_elapsed < TIME_LIMIT_P1 {

        robots
            .par_iter_mut()
            .for_each(|robot| {
                robot.update_position(num_x_tiles, num_y_tiles);
            });
        
        time_elapsed += 1;


    }

    calculate_robots(robots, num_x_tiles, num_y_tiles)
}

fn calculate_robots(robots: Vec<Robot>, num_x_tiles: usize, num_y_tiles: usize) -> usize {

    let mut quadrants: [usize; 4] = [0; 4];

    let x_mid: usize = num_x_tiles / 2;
    let y_mid: usize = num_y_tiles / 2;

    robots
        .iter()
        .for_each(|robot| {
            if robot.x_pos == x_mid || robot.y_pos == y_mid { return; }
            match (robot.x_pos < x_mid, robot.y_pos < y_mid) {
                (true, true)    => quadrants[0] += 1,
                (true, false)   => quadrants[1] += 1,
                (false, true)   => quadrants[2] += 1,
                (false, false)  => quadrants[3] += 1,

            }
        });

    quadrants
        .iter()
        .fold(1usize, |acc, count| acc * count)
}

pub fn part_two(input: &str, num_x_tiles: usize, num_y_tiles: usize) {

    let mut time_elapsed: usize = 0;

    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line
                .split(&['=', ',', ' '])
                .collect::<Vec<&str>>();

            Robot::new(
                split_line[1].parse::<usize>().ok().unwrap(),
                split_line[2].parse::<usize>().ok().unwrap(),
                split_line[4].parse::<isize>().ok().unwrap(),
                split_line[5].parse::<isize>().ok().unwrap()
            )
        })
        .collect::<Vec<Robot>>();

    while time_elapsed < TIME_LIMIT_P2 {

        let mut x_pos: Vec<usize> = Vec::new();
        let mut y_pos: Vec<usize> = Vec::new();

        for index in 0..robots.len() {
            robots[index].update_position(num_x_tiles, num_y_tiles);
            x_pos.push(robots[index].x_pos);
            y_pos.push(robots[index].y_pos);
        }
        
        time_elapsed += 1;

        if time_elapsed % 1000 == 0 {
            println!("{:?} seconds have elapsed", time_elapsed);
        }

        if (kstest(x_pos, num_x_tiles) < 0.01)
            && (kstest(y_pos, num_y_tiles) < 0.01) {

            println!("\n\n");
            println!("{:?} seconds have elapsed:", time_elapsed);
            visualize_grid(&robots, num_x_tiles, num_y_tiles);
            wait_for_keypress();
        }


    }
}

fn visualize_grid(robots: &Vec<Robot>, num_x_tiles: usize, num_y_tiles: usize) {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; num_x_tiles]; num_y_tiles];

    for robot in robots {
        grid[robot.y_pos][robot.x_pos] = 1;
    }
    let mut viz: String = String::new();

    for row in grid {
        for &count in &row {
            if count == 0 {
                viz.push('.');
            } else {
                viz.push('▆');
            }
        }
        viz.push('\n');
    }

    println!("{}", viz);
}

fn wait_for_keypress() {
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn kstest(mut rvs: Vec<usize>, num_tiles: usize) -> f64 {
    let range: f64 = (num_tiles + 1) as f64;

    rvs.sort_unstable();

    let n: f64 = rvs.len() as f64;
    let mut d_stat: f64 = 0.0;

    for (index, &value) in rvs.iter().enumerate() {
        let index   = index as f64;
        let cdf_value = ((value as f64 + 1.0) / range).clamp(0.0, 1.0);
        let d1 = ((index + 1.0) / n - cdf_value).abs();
        let d2 = (index / n - cdf_value).abs();
        d_stat = d_stat.max(d1).max(d2);
    }

     2.0 * (-2.0 * n * d_stat.powi(2)).exp()
}