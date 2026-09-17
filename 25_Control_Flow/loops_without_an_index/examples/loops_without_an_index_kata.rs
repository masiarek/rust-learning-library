//! Kata solution: five index loops rewritten without an index, each checked
//! against the original, and the one loop that should keep its indices.
//!
//!   rustc --edition 2024 loops_without_an_index_kata.rs -o /tmp/lwaik && /tmp/lwaik

fn main() {
    let lines = ["[package]", "name = \"demo\"", "", "[dependencies]"];
    let expected = ['b', 'a', 'd', 'c', 'a'];
    let actual = ['b', 'c', 'd', 'c', 'b'];
    let readings = [12, 15, 11, 18, 20, 19, 25];

    println!("1. Number the lines from 1");
    let mut by_index = Vec::new();
    for i in 0..lines.len() {
        by_index.push(format!("{}: {}", i + 1, lines[i]));
    }
    let by_enumerate: Vec<String> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{}: {line}", i + 1))
        .collect();
    let by_counter: Vec<String> = (1..).zip(&lines).map(|(n, line)| format!("{n}: {line}")).collect();
    println!("   {by_enumerate:?}");
    println!("   same as the index loop: {}; (1..).zip(&lines) gives the same again: {}", by_enumerate == by_index, by_counter == by_index);

    println!();
    println!("2. Count the answers that differ");
    let mut wrong_by_index = 0;
    for i in 0..expected.len() {
        if expected[i] != actual[i] {
            wrong_by_index += 1;
        }
    }
    let wrong = expected.iter().zip(&actual).filter(|(e, a)| e != a).count();
    println!("   zip + filter + count = {wrong}, index loop = {wrong_by_index}");
    let short = ['b', 'c'];
    let wrong_short = expected.iter().zip(&short).filter(|(e, a)| e != a).count();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let attempt = std::panic::catch_unwind(|| {
        let mut n = 0;
        for i in 0..expected.len() {
            if expected[i] != short[i] {
                n += 1;
            }
        }
        n
    });
    std::panic::set_hook(hook);
    let index_says = match attempt {
        Ok(n) => format!("{n}"),
        Err(_) => "a panic (index out of bounds)".to_string(),
    };
    println!("   against a 2-answer sheet: zip says {wrong_short} and stops quietly; the index loop gives {index_says}");

    println!();
    println!("3. The largest rise between consecutive readings");
    let mut max_by_index = i32::MIN;
    for i in 1..readings.len() {
        max_by_index = max_by_index.max(readings[i] - readings[i - 1]);
    }
    let max_rise = readings.windows(2).map(|w| w[1] - w[0]).max();
    println!("   windows(2) + max = {max_rise:?}, index loop = {max_by_index}");
    let one = [7];
    let mut one_by_index = i32::MIN;
    for i in 1..one.len() {
        one_by_index = one_by_index.max(one[i] - one[i - 1]);
    }
    let one_rise = one.windows(2).map(|w| w[1] - w[0]).max();
    println!("   on [7]: windows says {one_rise:?}; the index loop says {one_by_index}, a made-up number");

    println!();
    println!("4. Sum every third reading, starting with the first");
    let mut every_third_by_index = 0;
    let mut i = 0;
    while i < readings.len() {
        every_third_by_index += readings[i];
        i += 3;
    }
    let every_third: i32 = readings.iter().step_by(3).sum();
    println!("   step_by(3) + sum = {every_third}, index loop = {every_third_by_index}");

    println!();
    println!("5. Average each pair of readings");
    let mut averages_by_index = Vec::new();
    for i in (0..readings.len() - 1).step_by(2) {
        averages_by_index.push((readings[i] + readings[i + 1]) as f64 / 2.0);
    }
    let averages: Vec<f64> = readings
        .chunks(2)
        .map(|c| c.iter().sum::<i32>() as f64 / c.len() as f64)
        .collect();
    println!("   chunks(2)  -> {averages:?}");
    println!("   index loop -> {averages_by_index:?}   <- 7 readings: the index loop had to drop the last one");

    println!();
    println!("6. The one that keeps its indices: move every zero to the end, in place");
    let mut values = [0, 3, 0, 5, 7, 0, 2];
    let mut write = 0;
    for read in 0..values.len() {
        if values[read] != 0 {
            values.swap(write, read);
            write += 1;
        }
    }
    println!("   {values:?}");
    println!("   two positions move independently -- `read` every turn, `write` only on a keep --");
    println!("   and swap needs both at once. No adapter says that more clearly than two indices.");
}
