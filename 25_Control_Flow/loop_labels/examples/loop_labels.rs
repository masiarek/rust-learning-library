//! A label names a loop, or a block, so `break` and `continue` can say which.
//!
//!   rustc --edition 2024 loop_labels.rs -o /tmp/labels && /tmp/labels

struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("      drop: {}", self.0);
    }
}

fn main() {
    println!("1. break 'outer leaves every loop out to the label");
    let mut stopped_at = None;
    let mut xs_seen = Vec::new();
    let mut ys_seen = Vec::new();
    let mut innermost_passes = 0;
    'outer: for x in 0.. {
        xs_seen.push(x);
        for y in 0.. {
            ys_seen.push(y);
            for z in 0.. {
                innermost_passes += 1;
                if x + y + z > 1000 {
                    stopped_at = Some((x, y, z));
                    break 'outer;
                }
            }
        }
    }
    println!("   stopped at (x, y, z) = {stopped_at:?}");
    println!("   x values seen: {xs_seen:?}, y values seen: {ys_seen:?}, z passes: {innermost_passes}");
    println!("   The z loop alone carried the sum past 1000: x = 1 and y = 1 never happened.");

    let mut plain_ends = Vec::new();
    'cap: for x in 0.. {
        for y in 0..4 {
            // capped at 4 here; with the book's 0.. this loop never ends
            for z in 0.. {
                if x + y + z > 1000 {
                    plain_ends.push((x, y, z));
                    break; // leaves the z loop only
                }
            }
        }
        break 'cap;
    }
    println!("   plain break instead: {plain_ends:?} ...");
    println!("   Each break ends one z loop, and y just moves on.");

    println!();
    println!("2. The flag variable a label replaces, and two ways with no mut at all");
    let grid = [[1, 2, 3], [4, -5, 6], [-7, 8, 9]];

    let mut by_flag = None;
    let mut done = false;
    for (r, row) in grid.iter().enumerate() {
        for (c, &n) in row.iter().enumerate() {
            if n < 0 {
                by_flag = Some((r, c));
                done = true;
                break;
            }
        }
        if done {
            break; // the second check, one level out, that the flag needs
        }
    }

    let mut by_label = None;
    'rows: for (r, row) in grid.iter().enumerate() {
        for (c, &n) in row.iter().enumerate() {
            if n < 0 {
                by_label = Some((r, c));
                break 'rows;
            }
        }
    }

    let by_block = 'search: {
        for (r, row) in grid.iter().enumerate() {
            for (c, &n) in row.iter().enumerate() {
                if n < 0 {
                    break 'search Some((r, c));
                }
            }
        }
        None
    };

    let by_chain = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&n| n < 0).map(|c| (r, c)));

    println!("   flag variable      {by_flag:?}");
    println!("   break 'rows        {by_label:?}");
    println!("   labelled block     {by_block:?}");
    println!("   find_map           {by_chain:?}");

    println!();
    println!("3. continue 'batches abandons the rest of an outer pass");
    let batches = [[4, 7, 5], [6, -1, 8], [9, 2, 3]];
    let mut kept = Vec::new();
    let mut readings_checked = 0;
    'batches: for (b, batch) in batches.iter().enumerate() {
        for &reading in batch {
            readings_checked += 1;
            if reading < 0 {
                continue 'batches; // one bad reading discards the whole batch
            }
        }
        kept.push((b, batch.iter().sum::<i32>()));
    }
    println!("   kept (batch, sum) = {kept:?}");
    println!("   readings checked = {readings_checked} of 9   <- the reading after the -1 was never looked at");
    let kept_by_chain: Vec<(usize, i32)> = batches
        .iter()
        .enumerate()
        .filter(|(_, batch)| batch.iter().all(|&r| r >= 0))
        .map(|(b, batch)| (b, batch.iter().sum()))
        .collect();
    println!("   filter(all >= 0)  = {kept_by_chain:?}");

    println!();
    println!("4. A label is not a lifetime: one function uses 'a as both");
    println!("   first_long(&[\"an\", \"to\", \"apple\"]) = {:?}", first_long(&["an", "to", "apple"]));
    println!("   first_long(&[\"an\", \"to\"])          = {:?}", first_long(&["an", "to"]));

    println!();
    println!("5. A labelled block: break out of a block that is not a loop");
    for input in ["", "12x", "42"] {
        let verdict = 'check: {
            if input.is_empty() {
                break 'check "empty";
            }
            if !input.bytes().all(|b| b.is_ascii_digit()) {
                break 'check "not a number";
            }
            "ok"
        };
        println!("   {input:?} -> {verdict}");
    }

    println!();
    println!("6. No goto, and cleanup needs no jump");
    let goto = "an ordinary name";
    let r#do = "a reserved word, escaped";
    println!("   let goto = {goto:?};   let r#do = {:?};", r#do);
    for fail_at in [0, 1, 2] {
        println!("   load(fail_at = {fail_at}):");
        let result = load(fail_at);
        println!("      result: {result:?}");
    }
    println!("   The same shape without a function, as a labelled block:");
    let outcome = 'work: {
        let _file = Guard("close file");
        let _buffer = Guard("free buffer");
        if grid[2][0] < 0 {
            break 'work Err("negative cell");
        }
        Ok(grid[2][0])
    };
    println!("      outcome: {outcome:?}");
}

fn first_long<'a>(words: &[&'a str]) -> &'a str {
    'a: {
        for w in words {
            if w.len() > 3 {
                break 'a w;
            }
        }
        "(none)"
    }
}

fn load(fail_at: u8) -> Result<u32, String> {
    let _file = Guard("close file");
    if fail_at == 1 {
        return Err("could not open".to_string());
    }
    let _buffer = Guard("free buffer");
    let text = if fail_at == 2 { "4x" } else { "42" };
    let n: u32 = text.parse().map_err(|e| format!("{text:?}: {e}"))?;
    Ok(n)
}
