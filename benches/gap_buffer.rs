#![feature(test)]

extern crate test;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};
use test::Bencher;

const TESTS: [(usize, char); 100] = [
    (0, '5'),
    (1, '['),
    (1, 'i'),
    (3, 'k'),
    (4, ':'),
    (5, 'T'),
    (6, '='),
    (7, '$'),
    (8, '+'),
    (9, '-'),
    (10, 'C'),
    (11, 'W'),
    (11, 'm'),
    (13, '.'),
    (12, '9'),
    (15, '4'),
    (16, 'U'),
    (17, 'm'),
    (16, 'C'),
    (19, 'c'),
    (20, 'y'),
    (21, 'F'),
    (22, '_'),
    (23, '5'),
    (24, 't'),
    (25, 'z'),
    (26, 'V'),
    (27, 'B'),
    (28, '!'),
    (27, 'O'),
    (30, '|'),
    (31, 'O'),
    (32, 'T'),
    (33, 'S'),
    (34, '\''),
    (35, '*'),
    (36, 'P'),
    (37, '~'),
    (38, 'w'),
    (39, 'H'),
    (40, '^'),
    (41, '-'),
    (38, 'B'),
    (43, '"'),
    (43, ']'),
    (45, 'Q'),
    (46, 'b'),
    (47, 'W'),
    (48, 'W'),
    (49, 'i'),
    (0, '*'),
    (51, '*'),
    (52, 'd'),
    (53, 'H'),
    (54, '3'),
    (55, 'f'),
    (56, '&'),
    (57, 'a'),
    (58, 'Y'),
    (55, 'L'),
    (60, ')'),
    (57, '*'),
    (62, 'q'),
    (63, 'U'),
    (64, 'H'),
    (65, ':'),
    (66, 'b'),
    (66, 'k'),
    (68, 'w'),
    (69, 'z'),
    (70, 'M'),
    (71, 'K'),
    (68, 'v'),
    (73, 'J'),
    (74, '?'),
    (75, 'F'),
    (76, '"'),
    (77, 'Q'),
    (78, 'i'),
    (79, '0'),
    (80, 'h'),
    (81, '9'),
    (80, '('),
    (0, '2'),
    (84, '"'),
    (85, '2'),
    (86, 'D'),
    (87, 'g'),
    (88, '^'),
    (89, '}'),
    (85, '2'),
    (91, '@'),
    (92, 'o'),
    (93, 'x'),
    (94, '*'),
    (95, 'R'),
    (96, '%'),
    (97, 'r'),
    (98, '='),
    (99, 'M'),
];

trait InsertConvert: Default {
    fn insert(&mut self, index: usize, value: char);
    fn into_string(self) -> String;
}

#[derive(Default)]
struct RealVec(Vec<char>);

impl InsertConvert for RealVec {
    fn insert(&mut self, index: usize, value: char) {
        self.0.insert(index, value);
    }

    fn into_string(self) -> String {
        self.0.iter().collect()
    }
}

#[derive(Default)]
struct RealVecDeque(VecDeque<char>);

impl InsertConvert for RealVecDeque {
    fn insert(&mut self, index: usize, value: char) {
        self.0.insert(index, value);
    }

    fn into_string(self) -> String {
        self.0.iter().collect()
    }
}

#[derive(Default)]
struct RealBinaryHeap(BinaryHeap<Reverse<char>>);

impl InsertConvert for RealBinaryHeap {
    fn insert(&mut self, _index: usize, value: char) {
        self.0.push(Reverse(value));
    }

    fn into_string(self) -> String {
        let mut heap = self.0.into_sorted_vec();
        heap.reverse();
        heap.into_iter().map(|rev_char| rev_char.0).collect()
    }
}

fn test<T: InsertConvert>() {
    let mut data = T::default();
    for (index, value) in TESTS.iter() {
        data.insert(*index, *value);
    }
    let string = data.into_string();
    assert!(string.len() == 100);
}

macro_rules! make_benches {
    ($($fun:ident $type:ty)*) => {
        $(#[bench]
        fn $fun(b: &mut Bencher) {
            b.iter(|| {
                test::<$type>();
            })
        })*
    };
}

make_benches!(vec RealVec vec_deque RealVecDeque binary_heap RealBinaryHeap //gap_buffer RealGapBuffer
);

// from crate scribe
// struct RealGapBuffer(GapBuffer);

// impl Default for RealGapBuffer {
//     fn default() -> Self {
//         Self(GapBuffer::new(""))
//     }
// }

// impl InsertConvert for RealGapBuffer {
//     fn insert(&mut self, index: usize, value: char) {
//         self.0.insert(
//             value.to_string().as_str(),
//             &Position {
//                 line: 0,
//                 offset: index,
//             },
//         );
//     }

//     fn into_string(self) -> String {
//         self.0.to_string()
//     }
// }
