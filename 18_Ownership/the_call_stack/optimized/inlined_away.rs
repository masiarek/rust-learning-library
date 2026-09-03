/* The footnote, run rather than asserted: at -O the four frames are not there.
 *
 * This file is deliberately NOT in examples/, because its output is a property
 * of one optimizer on one machine and could never be an answer key. It is here
 * so the numbers on the lesson page can be reproduced instead of trusted.
 *
 *   rustc --edition 2024    inlined_away.rs -o /tmp/frames    && /tmp/frames
 *   rustc --edition 2024 -O inlined_away.rs -o /tmp/frames_O  && /tmp/frames_O
 *
 * Same four calls as the lesson's example, minus the #[inline(never)] that
 * holds them apart there.
 */

fn here(anchor: &u8) -> usize {
    anchor as *const u8 as usize
}

fn level_4(frames: &mut Vec<usize>) {
    let anchor = 0u8;
    frames.push(here(&anchor));
}
fn level_3(frames: &mut Vec<usize>) {
    let anchor = 0u8;
    frames.push(here(&anchor));
    level_4(frames);
}
fn level_2(frames: &mut Vec<usize>) {
    let anchor = 0u8;
    frames.push(here(&anchor));
    level_3(frames);
}
fn level_1(frames: &mut Vec<usize>) {
    let anchor = 0u8;
    frames.push(here(&anchor));
    level_2(frames);
}

fn main() {
    let anchor = 0u8;
    let main_frame = here(&anchor);
    let mut frames = Vec::new();
    level_1(&mut frames);

    println!("descending? {}", frames.windows(2).all(|w| w[1] < w[0]));
    println!("main -> deepest: {} bytes", main_frame.abs_diff(frames[3]));
    for (i, w) in frames.windows(2).enumerate() {
        println!("  level_{} -> level_{}: {} bytes", i + 1, i + 2, w[0].abs_diff(w[1]));
    }
}
