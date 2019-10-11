#[cfg(test)]
mod unit_tests;

use crate::Rolls;

const MAX_FRAMES_PER_GAME: usize = 10;
const MAX_ROLLS_PER_FRAME: usize = 2;
const MAX_PIN_SCORE_PER_FRAME: u8 = 10;
const ROLLS_PER_STRIKE: usize = 1;
const ROLLS_PER_SPARE: usize = 2;

pub fn score(rolls: &Rolls) -> u16 {
    score_by_frame(rolls.0.iter(), 1)
}

fn score_by_frame<'a, I>(mut iter: I, frame: usize) -> u16
where
    I: ExactSizeIterator<Item = &'a u8> + Clone,
{
    if frame > MAX_FRAMES_PER_GAME || iter.len() == 0 {
        return 0;
    }

    let mut frame_score = 0;

    // max `frame_score` is 10 which cannot overflow a `u8`
    // max `roll + 1` == 2 which cannot overflow a `u8`
    // max `.count() + 1` == 2 which cannot overflow a `u8`
    //     Note: `+ 1` req'd because `take_while()` 'consumes' final element (removed from src, but not in dst iterator
    #[allow(clippy::integer_arithmetic)]
    let rolls_this_frame = iter
        .by_ref()
        .enumerate()
        .take_while(|&(roll, &roll_score)| {
            frame_score += roll_score;
            frame_score < MAX_PIN_SCORE_PER_FRAME && roll + 1 < MAX_ROLLS_PER_FRAME
        })
        .count()
        + 1;

    let bonus_iter = iter.clone();
    let bonus_score = bonus_iter
        .take(match (frame_score, rolls_this_frame) {
            (MAX_PIN_SCORE_PER_FRAME, ROLLS_PER_STRIKE) => 2,
            (MAX_PIN_SCORE_PER_FRAME, ROLLS_PER_SPARE) => 1,
            _ => 0,
        })
        .sum::<u8>();

    // max `frame_score + bonus score` == 30 which cannot overflow a `u8`
    // max cumulative score for rest of game == 270 which cannot overflow a `u16`
    // ∴ max cumulative score == 300 which cannot overflow a `u16`
    #[allow(clippy::integer_arithmetic)]
    // braces due to rustc: 1.38.0: error[E0658]: attributes on expressions are experimental
    {
        u16::from(frame_score + bonus_score) + score_by_frame(iter, frame + 1)
    }
}
