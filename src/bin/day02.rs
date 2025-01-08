use adventofcode2016::build_main;

fn part1(input: &str) -> usize {
    // Order: URDL
    let graph = [
        [0, 0, 0, 0],
        [1, 2, 4, 1],
        [2, 3, 5, 1],
        [3, 3, 6, 2],
        [1, 5, 7, 4],
        [2, 6, 8, 4],
        [3, 6, 9, 5],
        [4, 8, 7, 7],
        [5, 9, 8, 7],
        [6, 9, 9, 8]
    ];

    let mut cur = 5;
    let mut result = 0;

    for line in input.lines() {
        for c in line.chars() {
            let i = match c { 'U' => 0, 'R' => 1, 'D' => 2, 'L' => 3, _ => panic!() };
            cur = graph[cur][i];
        }
        result = 10 * result + cur;
    }

    result
}

fn part2(input: &str) -> String {
    // Order: URDL
    let graph = [
        [0, 0, 0, 0],
        [1, 1, 3, 1],
        [2, 3, 6, 2],
        [1, 4, 7, 2],
        [4, 4, 8, 3],
        [5, 6, 5, 5],
        [2, 7, 10, 5],
        [3, 8, 11, 6],
        [4, 9, 12, 7],
        [9, 9, 9, 8],
        [6, 11, 10, 10],
        [7, 12, 13, 10],
        [8, 12, 12, 11],
        [11, 13, 13, 13]
    ];
    let chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D'];

    let mut result = String::new();
    let mut cur = 5;

    for line in input.lines() {
        for c in line.chars() {
            let i = match c { 'U' => 0, 'R' => 1, 'D' => 2, 'L' => 3, _ => panic!() };
            cur = graph[cur][i];
        }
        result.push(chars[cur]);
    }

    result
}

build_main!("day02.txt", "Part 1" => part1, "Part 2" => part2);