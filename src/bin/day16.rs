/*
   If our string is A, and its reversed complement is B, a pattern quickly emerges:
   - A
   - A[0]B
   - [A0B]0[A1B]
   - [A0B0A1B]0[A0B1A1B]
   - [A0B0A1B0A0B1A1B]0[A0B0A1B1A0B1A1B]
   - [A0B0A1B0A0B1A1B0A0B0A1B1A0B1A1B]0[A0B0A1B0A0B1A1B1A0B0A1B1A0B1A1B]
   - ...

   The patterns are:
   1. The string consists of alternating A's and B's with "connector" characters in between
   2. The initial segment is always the entire previous string; so, these can all just be viewed
        as initial segments of an infinite sequence
   3. The "connector" characters themselves form the dragon curve sequence for seed '0', by
        construction:
        - 0
        - [0]0[1]
        - [001]0[011]
        - [0010011]0[0011011]
        - [001001100011011]0[001001110011011]
        - ...

    So, if we can define an iterator for the "base" dragon sequence (the connectors), we can use
    that to define an efficient iterator for the entire sequence by simply iterating over the
    string forward, then a connector, then backward complement, then the next connector, then ...

    By construction, term i (starting from 0) of the "base" sequence looks as follows:
    - If i % 4 == 0, you are reading '0' forward (so it is a 0)
    - If i % 4 == 2, you are reading '0' in backward complement (so it is a 1)
    - Otherwise, you are in a connector. But by the above, the connector term at i=2n+1 is simply
        the original sequence term at n, so check it mod 4.

    This is nice, but requires you to do iterative computation of n = (i - 1) / 2. On the other
    hand, if we consider 1-indexing instead:
    - If i % 4 == 1, you are reading '0' forward (so it is a 0)
    - If i % 4 == 3, you are reading '0' in backward complement (so it is a 1)
    - Otherwise, you are in a connector.  But by the above, the term at i=2n is simply the
        original sequence term at n, so check it mod 4.

    This means that if i = 2^k*m where m is odd, you need only think about m % 4 = 1 or 3. And we
    can efficiently compute 2^k and m by the bitwise trick 2^k = i & (!i + 1) and m = n / 2^k.

    To go back to 0-indexing, just use i + 1 = 2^k*m.

    So, now we need to compute the checksum. If we want n=2^km digits of the sequence, the checksum
    will have m digits: the first digit of the checksum will be determined by the first 2^k digits,
    the next by the next 2^k, etc.

    Claim: an element of the checksum is 1 precisely when the corresponding chunk of 2^k sequence
    elements contains an even number of 0's.

    Proof: When k=0, this is obvious: the checksum is the original sequence, and it is a 1 if its
    only element is 1.

    If k > 0, we do a compression phase.  Suppose the 2^k elements consist of n0 0's and n1 1's,
    and the pairs of elements consist of n00 '00's, n11 '11's, and n01 of either '01' or 10'. Then
    the compressed sequence will contain n00 + n11 1's, and n01 0's. But n0 = 2*n00 + n01, and
    n1 = 2*n11 + n01; so, n0 is even if and only if n01 is even. Thus compression conserves the
    parity of the number of 0's, and inductively the claim holds.
 */

use itertools::Itertools;
use adventofcode2016::build_main;

struct BaseDragonCurve {
    i: usize
}

impl BaseDragonCurve {
    fn new() -> BaseDragonCurve { BaseDragonCurve { i: 0 }}
}

impl Iterator for BaseDragonCurve {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.i + 1;
        let power_of_two = x & (!x + 1);
        let odd = x / power_of_two;

        self.i += 1;

        if odd % 4 == 1 { Some('0') } else { Some('1') }
    }
}

#[derive(Copy, Clone)]
enum State { Forward(usize), Backward(usize), Connector(bool) }
struct ModifiedDragonCurve {
    state: State,
    connectors: BaseDragonCurve,
    seed: Vec<char>
}

impl ModifiedDragonCurve {
    fn new(seed: &str) -> ModifiedDragonCurve {
        let state = State::Forward(0);
        let connectors = BaseDragonCurve::new();
        let seed = seed.chars().collect_vec();

        ModifiedDragonCurve { state, connectors, seed }
    }
}

impl Iterator for ModifiedDragonCurve {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Forward(i) => {
                let result = Some(self.seed[i]);
                if i + 1 == self.seed.len() {
                    self.state = State::Connector(false);
                }
                else {
                    self.state = State::Forward(i + 1);
                }
                result
            },
            State::Connector(forward_next) => {
                let result = self.connectors.next();
                self.state = match forward_next {
                    true => State::Forward(0),
                    false => State::Backward(self.seed.len() - 1)
                };
                result
            },
            State::Backward(i) => {
                let result = Some(if self.seed[i] == '1' { '0' } else { '1' });

                if i == 0 {
                    self.state = State::Connector(true);
                }
                else {
                    self.state = State::Backward(i - 1);
                }

                result
            }
        }
    }
}

fn build_checksum<I>(iter: I, for_bits: usize) -> String where I: Iterator<Item = char>{
    let even_part = for_bits & (!for_bits + 1);
    let odd_part = for_bits / even_part;

    iter.chunks(even_part).into_iter()
        .map(|chunk| chunk.filter(|&c| c == '0').count())
        .map(|count| if count % 2 == 0 { '1' } else { '0' })
        .take(odd_part)
        .collect()
}

fn part1(input: &str) -> String {
    let seed = input.trim();
    let c = ModifiedDragonCurve::new(seed);
    build_checksum(c, 272)
}

fn part2(input: &str) -> String {
    let seed = input.trim();
    let c = ModifiedDragonCurve::new(seed);
    build_checksum(c, 35651584)
}

build_main!("day16.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_curve() {
        let mut c = BaseDragonCurve::new();

        let s: String = c.take(31).collect();

        assert_eq!(s, "0010011000110110001001110011011");

        c = BaseDragonCurve { i: 0 };

        println!("{}", c.nth(100000).unwrap())
    }

    #[test]
    fn test_modified_curve() {
        let c = ModifiedDragonCurve::new("10000");

        let s: String = c.take(23).collect();
        assert_eq!(s, "10000011110010000111110");
    }
}