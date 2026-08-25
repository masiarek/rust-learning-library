//! Kata solution: a tic-tac-toe position in 18 bits, plus a move counter above it.
//!
//! bits  0..8  X cells      bits 9..17  O cells      bits 18..21  moves played
//! Cells are numbered in reading order, cell n is bit n.

const CELLS: u32 = 0x1FF; // nine bits
const O_SHIFT: u32 = 9;
const MOVES_SHIFT: u32 = 18;
const MOVES_MASK: u32 = 0xF;

const WINS: [(u32, &str); 8] = [
    (0b000_000_111, "top row"),
    (0b000_111_000, "middle row"),
    (0b111_000_000, "bottom row"),
    (0b001_001_001, "left column"),
    (0b010_010_010, "middle column"),
    (0b100_100_100, "right column"),
    (0b100_010_001, "diagonal \\"),
    (0b001_010_100, "diagonal /"),
];

#[derive(Clone, Copy, Default)]
struct Board(u32);

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl Board {
    fn cells(self, p: Player) -> u32 {
        match p {
            Player::X => self.0 & CELLS,
            Player::O => (self.0 >> O_SHIFT) & CELLS, // the mask is load-bearing
        }
    }
    fn occupied(self) -> u32 {
        self.cells(Player::X) | self.cells(Player::O)
    }
    fn moves(self) -> u32 {
        (self.0 >> MOVES_SHIFT) & MOVES_MASK
    }
    fn mark(self, p: Player, cell: u32) -> Board {
        let bit = 1 << cell;
        if self.occupied() & bit != 0 {
            return self; // already taken
        }
        let placed = match p {
            Player::X => self.0 | bit,
            Player::O => self.0 | (bit << O_SHIFT),
        };
        // bump the counter field without disturbing the two below it
        let n = ((placed >> MOVES_SHIFT) & MOVES_MASK) + 1;
        Board((placed & !(MOVES_MASK << MOVES_SHIFT)) | (n << MOVES_SHIFT))
    }
    fn winner(self) -> Option<(Player, &'static str)> {
        for p in [Player::X, Player::O] {
            let mine = self.cells(p);
            for (mask, name) in WINS {
                if mine & mask == mask {
                    return Some((p, name));
                }
            }
        }
        None
    }
    fn render(self) -> String {
        let (x, o) = (self.cells(Player::X), self.cells(Player::O));
        (0..9)
            .map(|c| {
                let ch = if x >> c & 1 == 1 {
                    'X'
                } else if o >> c & 1 == 1 {
                    'O'
                } else {
                    '.'
                };
                if c % 3 == 2 { format!("{ch}\n  ") } else { format!("{ch} ") }
            })
            .collect::<String>()
    }
}

fn main() {
    println!("=== a game, played into one u32 ===");
    let moves = [
        (Player::X, 0u32),
        (Player::O, 1),
        (Player::X, 4),
        (Player::O, 2),
        (Player::X, 8),
    ];
    let mut b = Board::default();
    for (p, cell) in moves {
        b = b.mark(p, cell);
    }
    println!("  {}", b.render().trim_end());
    let raw = format!("{:022b}", b.0);
    println!("  raw            = {raw}");
    println!("  the same bits  = {} {} {}   <- moves | O cells | X cells", &raw[0..4], &raw[4..13], &raw[13..22]);
    println!("  X cells        = {:09b}", b.cells(Player::X));
    println!("  O cells        = {:09b}", b.cells(Player::O));
    println!("  moves played   = {}", b.moves());
    println!("  occupied       = {:09b}  ({} of 9)", b.occupied(), b.occupied().count_ones());

    println!("\n=== the win check is one mask comparison ===");
    let x = b.cells(Player::X);
    for (mask, name) in WINS {
        println!("  x & {mask:09b} == mask  ->  {:<5}  {name}", x & mask == mask);
    }
    match b.winner() {
        Some((p, how)) => println!("  winner: {} on the {how}", if p == Player::X { 'X' } else { 'O' }),
        None => println!("  no winner yet"),
    }

    println!("\n=== the trap, once a third field sits above the other two ===");
    println!("  O cells  (b >> {O_SHIFT}) & 0x1FF = {:09b}   right", (b.0 >> O_SHIFT) & CELLS);
    println!("  O cells   b >> {O_SHIFT}          = {:09b}   WRONG once moves is nonzero",
        (b.0 >> O_SHIFT) & 0x1FFF);
    println!("  ...the move counter is sitting directly on top of O, exactly where");
    println!("     the TCP offset sat on top of reserved. Same bug, different header.");
    println!("  X cells   b & 0x1FF       = {:09b}   no shift needed: X is the bottom field", b.0 & CELLS);
}
