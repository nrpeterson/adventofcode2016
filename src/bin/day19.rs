use adventofcode2016::build_main;

fn part1(input: &str) -> isize {
    let mut k = input.parse::<isize>().unwrap();
    let mut a = 1isize;
    let mut b = 0isize;

    while k > 1 {
        if k % 2 == 0 {
            // We go from e(1),e(2),e(3),...,e(k) to e(1),e(3),e(5),...,e(k-1)=f(1),f(2),...,f(m)
            // where f(i)=e(2*i-1), 2*m-1=k-1 => 2*m=k => m = k/2.
            // If e(i)=ax+b, then f(i)=e(2i-1)=a(2i-1)+b=2ai+(b-a).
            (a, b, k) = (2 * a, b - a, k / 2);
        }
        else {
            // We go from e(1),e(2),e(3),...,e(k) to e(3), e(5), ..., e(m)= f(1), f(2), ..., f(k)
            // where f(i)=e(2i+1) and m=2k+1 => k=(m-1)/2.
            // If e(i) = a*i + b, then f(i)=e(2*i+1)=a*(2*i+1)+b=2ai+(a+b)
            (a, b, k) = (2 * a, b + a, (k - 1) / 2);
        }
    }

    a + b
}

fn part2(input: &str) -> isize {
    // https://oeis.org/A334473
    let n = input.parse::<isize>().unwrap();
    let mut pow3 = 1;
    while 3 * pow3 <= n {
        pow3 *= 3;
    }
    let b = n - pow3;

    match b {
        0 => pow3,
        b if b <= pow3 => b,
        _ => 2*b - pow3
    }
}

build_main!("day19.txt", "Part 1" => part1, "Part 2" => part2);