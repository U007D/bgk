#[cfg(test)]
mod unit_tests;

use crate::Rolls;

const MAX_ROLLS_PER_FRAME: usize = 2;
const MAX_PIN_SCORE_PER_FRAME: u8 = 10;

#[allow(clippy::needless_pass_by_value)]
pub fn score(rolls: Rolls) -> u16 { score_by_frame(rolls.0.iter(), 1) }

fn score_by_frame<'a, I>(mut iter: I, frame: usize) -> u16 where I: ExactSizeIterator<Item = &'a u8> + Clone {
    if iter.len() == 0 || frame > 10 {
        return 0
    }

    let mut frame_score = 0;

    // max `frame_score` is 10 which cannot overflow a `u8`
    // max `i + 1` == 2 which cannot overflow a `u8`
    // max `count + 1` == 2 which cannot overflow a `u8`
    //     Note: `+ 1` req'd because `take_while()` 'consumes' final element (removed from src, but not in dst iterator
    #[allow(clippy::integer_arithmetic)]
    let rolls_this_frame = iter.by_ref()
                               .enumerate()
                               .take_while(|&(i, &v)| {
                                   frame_score += v;
                                   frame_score < MAX_PIN_SCORE_PER_FRAME && i + 1 < MAX_ROLLS_PER_FRAME
                               })
                               .count() + 1;

    let bonus_iter = iter.clone();
    let bonus_score = bonus_iter.take(match (frame_score, rolls_this_frame) {
                                    (MAX_PIN_SCORE_PER_FRAME, 1) => 2,
                                    (MAX_PIN_SCORE_PER_FRAME, 2) => 1,
                                    _ => 0,
                                })
                                .sum::<u8>();

    // max `frame_score + bonus score` == 30 which cannot overflow a `u8`
    // max cumulative score for rest of game == 270 which cannot overflow a `u16`
    #[allow(clippy::integer_arithmetic)]
    { u16::from(frame_score + bonus_score) + score_by_frame(iter, frame + 1) }
}
