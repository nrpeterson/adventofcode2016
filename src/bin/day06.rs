use std::ops::{Index, IndexMut};
use adventofcode2016::build_main;

#[derive(Clone)]
struct CharMap<T> {
    data: [T; 26]
}

impl<T> CharMap<T> where T: Copy {
    fn new(init: T) -> CharMap<T> {
        let data = [init; 26];
        CharMap { data }
    }
}

impl<T> CharMap<T> where T: Ord + Copy {
    fn max_char(&self) -> char {
        (b'a'..=b'z').map(char::from)
            .max_by_key(|c| self[*c])
            .unwrap()
    }

    fn min_char(&self) -> char {
        (b'a'..=b'z').map(char::from)
            .min_by_key(|c| self[*c])
            .unwrap()
    }
}

impl<T> Index<char> for CharMap<T> {
    type Output = T;

    fn index(&self, index: char) -> &Self::Output {
        let i = (index as usize) - ('a' as usize);
        &self.data[i]
    }
}

impl<T> IndexMut<char> for CharMap<T> {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        let i = (index as usize) - ('a' as usize);
        &mut self.data[i]
    }
}

fn counts(input: &str) -> Vec<CharMap<usize>> {
    let n = input.lines().next().unwrap().len();

    input.lines()
        .fold(vec![CharMap::new(0); n], |mut acc, line| {
            line.chars()
                .zip(acc.iter_mut())
                .for_each(|(c, counts)| { counts[c] += 1; });

            acc
        })
}

fn part1(input: &str) -> String {
    counts(input)
        .iter()
        .map(|counts| counts.max_char())
        .collect::<String>()
}

fn part2(input: &str) -> String {
    counts(input)
        .iter()
        .map(|counts| counts.min_char())
        .collect::<String>()
}

build_main!("day06.txt", "Part 1" => part1, "Part 2" => part2);