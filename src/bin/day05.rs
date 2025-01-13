use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
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

fn part1(input: &str) -> String {
    HashIter::new(input.trim())
        .filter_map(|hash| {
            if hash.chars().take(5).all(|c| c == '0') {
                hash.chars().nth(5)
            }
            else {
                None
            }
        })
        .take(8)
        .collect::<String>()
}

fn part2(input: &str) -> String {
    HashIter::new(input.trim())
        .filter_map(|hash| {
            if hash.chars().take(5).all(|c| c == '0') {
                let i = (hash.chars().nth(5).unwrap() as usize) - ('0' as usize);
                let c = hash.chars().nth(6).unwrap();
                if i < 8 { Some((i, c)) } else { None }
            }
            else {
                None
            }
        })
        .fold_while([None; 8], |mut acc, (i, c)| {
            if acc.iter().all(|x| x.is_some()) { return Done(acc) }
            acc[i] = acc[i].or(Some(c));
            Continue(acc)
        })
        .into_inner()
        .iter()
        .map(|x| x.unwrap())
        .collect::<String>()

}

build_main!("day05.txt", "Part 1" => part1, "Part 2" => part2);