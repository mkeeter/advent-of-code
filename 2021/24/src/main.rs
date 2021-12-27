use rayon::prelude::*;

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

fn main() {
    let mut state = vec![([0; 4], (0, 0))];

    for (f, r) in PASSES.iter().zip(INPUTS) {
        println!("{}", state.len());
        // Clear the register that's about to be written
        state.par_iter_mut().for_each(|k| k.0[r] = 0);

        // Sort by register state, then do single-pass compaction
        state.par_sort_unstable_by_key(|k| k.0);
        let mut i = 0;
        let mut j = 1;
        while j < state.len() {
            if state[i].0 == state[j].0 {
                let (imin, imax) = state[i].1;
                let (jmin, jmax) = state[j].1;
                state[i].1 = (imin.min(jmin), imax.max(jmax));
            } else {
                i += 1;
                state[i] = state[j];
            }
            j += 1;
        }
        assert!(i < state.len());
        state.resize(i + 1, ([0; 4], (0, 0)));

        state = (1..=9)
            .into_par_iter()
            .flat_map(|i| {
                state.par_iter().map(move |(regs, (min, max))| {
                    (f(*regs, i), (min * 10 + i as usize, max * 10 + i as usize))
                })
            })
            .collect();
    }

    let (min, max) = state
        .par_iter()
        .filter(|(k, _)| k[2] == 0)
        .map(|(_, v)| *v)
        .reduce(|| (usize::MAX, 0), |a, b| (a.0.min(b.0), a.1.max(b.1)));
    println!("Part 1: {}                         ", max);
    println!("Part 2: {}                         ", min);
}
