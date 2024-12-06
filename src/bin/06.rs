use hashbrown::HashSet;

fn find_start(m: &[Vec<u8>]) -> (usize, usize) {
    for r in 0..m.len() {
        for c in 0..m[0].len() {
            if m[r][c] == b'^' {
                return (r, c);
            }
        }
    }
    unreachable!()
}

fn walk(m: &[Vec<u8>], mut r: usize, mut c: usize) -> Option<HashSet<(usize, usize)>> {
    let mut seen = HashSet::new();
    let mut d = 0;
    loop {
        if !seen.insert((r, c, d)) {
            return None;
        }
        let (dr, dc) = [(-1,0), (0,1), (1,0), (0, -1)][d];
        let (rr, cc) = (r + dr as usize, c + dc as usize);
        if !(0..m.len()).contains(&rr) || !(0..m[0].len()).contains(&cc) {
            return Some(seen.iter().map(|&(r, c, _)| (r, c)).collect());
        }
        if m[rr][cc] == b'#' {
            d = (d + 1) % 4;
        } else {
            (r, c) = (rr, cc);
        }
    }
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let mut m = input.lines().map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>();
    let (sr, sc) = find_start(&m);
    let p1 = walk(&m, sr, sc).unwrap();
    let p2 = p1.iter().filter(|&&(r, c)| {
        let saved = m[r][c];
        m[r][c] = b'#';
        let ok = walk(&m, sr, sc).is_none();
        m[r][c] = saved;
        ok
    }).count();
    (p1.len(), p2)
}
