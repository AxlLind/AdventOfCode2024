use std::collections::VecDeque;
use hashbrown::HashSet;

fn reachable_nines(g: &[&[u8]], r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut q = VecDeque::from([(r, c)]);
    let mut seen = Vec::new();
    while let Some((r, c)) = q.pop_front() {
        if g[r][c] == b'9' {
            seen.push((r, c));
            continue;
        }
        for (rr, cc) in [(r+1, c), (r-1, c), (r, c+1), (r, c-1)] {
            if *g.get(rr).and_then(|row| row.get(cc)).unwrap_or(&0) == g[r][c] + 1 {
                q.push_back((rr, cc));
            }
        }
    }
    seen
}

#[aoc::main(10)]
fn main(input: &str) -> (usize, usize) {
    let g = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let (mut p1, mut p2) = (0, 0);
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            if g[r][c] == b'0' {
                let seen = reachable_nines(&g, r, c);
                p1 += seen.iter().collect::<HashSet<_>>().len();
                p2 += seen.len();
            }
        }
    }
    (p1, p2)
}
