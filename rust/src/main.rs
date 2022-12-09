mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use crate::day_1::{day_1_part_1, day_1_part_2, input::INPUT_DAY_1};
use crate::day_2::{day_2_part_1, day_2_part_2, input::INPUT_DAY_2};
use crate::day_3::{day_3_part_1, day_3_part_2, input::INPUT_DAY_3};
use crate::day_4::{day_4_part_1, day_4_part_2, input::INPUT_DAY_4};
use crate::day_5::{
    day_5_part_1, day_5_part_2,
    input::{INPUT_DAY_5_MOVES, INPUT_DAY_5_STACKS},
};
use crate::day_6::{day_6_part_1, day_6_part_2, input::INPUT_DAY_6};
use crate::day_7::{day_7_part_1, day_7_part_2, input::INPUT_DAY_7};
use crate::day_8::{day_8_part_1, day_8_part_2, input::INPUT_DAY_8};
use crate::day_9::{day_9_part_1, day_9_part_2, input::INPUT_DAY_9};

fn main() {
    let d1p1 = day_1_part_1(INPUT_DAY_1);
    let d1p2 = day_1_part_2(INPUT_DAY_1);

    dbg!("{}", d1p1);
    dbg!("{}", d1p2);

    let d2p1 = day_2_part_1(INPUT_DAY_2);
    let d2p2 = day_2_part_2(INPUT_DAY_2);

    dbg!("{}", d2p1);
    dbg!("{}", d2p2);

    let d3p1 = day_3_part_1(INPUT_DAY_3);
    let d3p2 = day_3_part_2(INPUT_DAY_3);

    dbg!("{}", d3p1);
    dbg!("{}", d3p2);

    let d4p1 = day_4_part_1(INPUT_DAY_4);
    let d4p2 = day_4_part_2(INPUT_DAY_4);

    dbg!("{}", d4p1);
    dbg!("{}", d4p2);

    let d5p1 = day_5_part_1(INPUT_DAY_5_STACKS, INPUT_DAY_5_MOVES);
    let d5p2 = day_5_part_2(INPUT_DAY_5_STACKS, INPUT_DAY_5_MOVES);

    dbg!("{}", d5p1);
    dbg!("{}", d5p2);

    let d6p1 = day_6_part_1(INPUT_DAY_6);
    let d6p2 = day_6_part_2(INPUT_DAY_6);

    dbg!("{}", d6p1);
    dbg!("{}", d6p2);

    let d7p1 = day_7_part_1(INPUT_DAY_7);
    let d7p2 = day_7_part_2(INPUT_DAY_7);

    dbg!("{}", d7p1);
    dbg!("{}", d7p2);

    let d8p1 = day_8_part_1(INPUT_DAY_8);
    let d8p2 = day_8_part_2(INPUT_DAY_8);

    dbg!("{}", d8p1);
    dbg!("{}", d8p2);

    let d9p1 = day_9_part_1(INPUT_DAY_9);
    let d9p2 = day_9_part_2(INPUT_DAY_9);

    dbg!("{}", d9p1);
    dbg!("{}", d9p2);
}
