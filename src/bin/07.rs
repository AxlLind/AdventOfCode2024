fn is_valid(target: i64, ns: &[(i64, &str)], n: i64, part_two: bool) -> bool {
    if ns.is_empty() || n > target {
        return n == target;
    }
    if is_valid(target, &ns[1..], n + ns[0].0, part_two) {
        return true;
    }
    if is_valid(target, &ns[1..], n * ns[0].0, part_two) {
        return true;
    }
    if part_two {
        let n = (n.to_string() + ns[0].1).parse::<i64>().unwrap();
        if is_valid(target, &ns[1..], n, part_two) {
            return true;
        }
    }
    false
}

#[aoc::main(07)]
fn main(input: &str) -> (i64, i64) {
    let ops = input.lines().map(|l| {
        let (n, rest) = l.split_once(": ").unwrap();
        let ns = rest.split(' ').map(|w| (w.parse().unwrap(), w)).collect::<Vec<(i64, _)>>();
        (n.parse::<i64>().unwrap(), ns)
    });

    let (mut p1, mut p2) = (0, 0);
    for (n, ns) in ops {
        if is_valid(n, &ns, 0, false) {
            p1 += n;
        }
        if is_valid(n, &ns, 0, true) {
            p2 += n;
        }
    }
    (p1, p2)
}