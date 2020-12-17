#![feature(min_const_generics)]

mod map;

use map::{Coord, Cube, Map};
use std::{collections::HashMap, fs, mem};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let active_cubes = run_6_cycles(parse_input::<3>(input));
    println!("Part 1 active cubes after 6 cycles: {}", active_cubes);
}

fn part2(input: &str) {
    let active_cubes = run_6_cycles(parse_input::<4>(input));
    println!("Part 1 active cubes after 6 cycles: {}", active_cubes);
}

fn run_6_cycles<const N: usize>(mut map: Map<N>) -> usize {
    let mut buffer = Map::new();
    for _ in 0..6 {
        update_map(&mut map, &mut buffer);
        map.clear();
        mem::swap(&mut map, &mut buffer);
    }
    map.active_cubes().count()
}

fn update_map<const N: usize>(map: &mut Map<N>, write_buffer: &mut Map<N>) {
    let mut neighbor_tracker = HashMap::new();

    for coord in map.active_cubes() {
        for neighbor_coord in map.neighbors_of(coord) {
            *neighbor_tracker.entry(neighbor_coord).or_insert(0) += 1;
        }
    }
    for (coord, active_neighbour_count) in neighbor_tracker.drain() {
        let cube = map.get_cube(coord);
        let cube = match cube {
            Cube::Active => {
                if (2..=3).contains(&active_neighbour_count) {
                    Cube::Active
                } else {
                    Cube::Inactive
                }
            }
            Cube::Inactive => {
                if active_neighbour_count == 3 {
                    Cube::Active
                } else {
                    Cube::Inactive
                }
            }
        };
        write_buffer.set_cube(coord, cube);
    }
}

fn parse_input<const N: usize>(input: &str) -> Map<N> {
    input
        .lines()
        .zip(0_i32..)
        .flat_map(|(line, y)| {
            line.chars()
                .map(|char| match char {
                    '#' => Cube::Active,
                    '.' => Cube::Inactive,
                    _ => panic!("Invalid input"),
                })
                .zip(0_i32..)
                .map(move |(cube, x)| {
                    let mut dimensions = [0; N];
                    dimensions[0] = x;
                    dimensions[1] = y;
                    (Coord { dimensions }, cube)
                })
        })
        .collect::<Map<N>>()
}
