use std::collections::VecDeque;
use itertools::Itertools;
use md5;
use adventofcode2016::build_main;

struct HashIter {
    salt_len: usize,
    cache: String,
    i: usize
}

impl HashIter {
    fn new(salt: &str) -> HashIter {
        HashIter {
            salt_len: salt.len(),
            cache: salt.to_string(),
            i: 0
        }
    }
}

impl Iterator for HashIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.cache.drain(self.salt_len..);
        self.cache.push_str(&self.i.to_string());
        self.i += 1;
        Some(format!("{:x}", md5::compute(&self.cache)))
    }
}

struct Windowed<I> where I: Iterator {
    it: I,
    n: usize,
    buffer: VecDeque<I::Item>
}

impl<I> Windowed<I> where I: Iterator {
    fn new(mut it: I, n: usize) -> Windowed<I> {
        let mut buffer: VecDeque<I::Item> = VecDeque::with_capacity(n);

        (0..n).for_each(|_| {
            let elem = it.next();
            match elem {
                Some(e) => buffer.push_back(e),
                None => panic!("SHIT")
            }
        });

        Windowed { it, n, buffer }
    }
}

impl<I> Iterator for Windowed<I> where I: Iterator, I::Item: Clone {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.len() < self.n {
            return None
        }

        let result: Option<Vec<I::Item>> = Some(self.buffer.iter().cloned().collect());
        self.buffer.pop_front();
        self.it.next().into_iter().for_each(|c| self.buffer.push_back(c));

        result
    }
}

fn first_triple(s: &str) -> Option<char> {
    s.chars().tuple_windows()
        .find(|&(a, b, c)| a == b && b == c)
        .map(|(a, _, _)| a)
}

fn contains_quintet(c: char, s: &str) -> bool {
    s.contains(&format!("{c}{c}{c}{c}{c}"))
}

fn find_keys<I>(it: I) -> usize where I: Iterator<Item=String> {
    let windows = Windowed::new(it, 1001);

    windows.enumerate()
        .filter(|(_, window)| {
            match first_triple(&window[0]) {
                Some(c) => {
                    window[1..].iter().any(|tail| contains_quintet(c, tail))
                },
                None => false
            }
        })
        .map(|(i, _)| i)
        .nth(63)
        .unwrap()
}

fn part1(input: &str) -> usize {
    let it = HashIter::new(input.trim());
    find_keys(it)
}

fn part2(input: &str) -> usize {
    let it = HashIter::new(input.trim())
        .map(|s| {
            (0..2016).fold(s, |prev, _| {
                format!("{:x}", md5::compute(&prev))
            })
        });

    find_keys(it)
}

build_main!("day14.txt", "Part 1" => part1, "Part 2" => part2);