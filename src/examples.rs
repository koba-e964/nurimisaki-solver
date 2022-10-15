use core::num::NonZeroU8;

use crate::square::{parse_from_puzz_link, Square};

pub fn example0() -> Vec<Vec<Square>> {
    // https://www.nikoli.co.jp/ja/puzzles/nurimisaki/
    let mut board = vec![
        vec![
            Square::Number(NonZeroU8::new(3).unwrap()),
            Square::Blank,
            Square::Number(NonZeroU8::new(1).unwrap()),
            Square::Blank,
            Square::Blank,
        ],
        vec![Square::Blank; 5],
        vec![Square::Blank; 5],
        vec![Square::Blank; 5],
        vec![Square::Blank; 5],
    ];
    for (x, y) in [(1, 3), (2, 0), (4, 4)] {
        board[x][y] = Square::Number(NonZeroU8::new(4).unwrap());
    }
    board
}

pub fn example1() -> Vec<Vec<Square>> {
    // https://puzsq.logicpuzzle.app/puzzle/101318
    parse_from_puzz_link("https://puzz.link/p?nurimisaki/6/6/4j6l.o.j2h4i.g").unwrap()
}

pub fn example2() -> Vec<Vec<Square>> {
    // https://puzsq.logicpuzzle.app/puzzle/101809
    parse_from_puzz_link("https://puzz.link/p?nurimisaki/10/10/g2t.h.r.j.u6l.t4zg.g6").unwrap()
}
