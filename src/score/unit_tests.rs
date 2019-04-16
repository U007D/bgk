use super::*;
use crate::Result;

#[test]
fn calc_no_rolls_returns_0() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[])?;

    // when invoked
    let res = score(rolls);

    // the score should be 0
    assert_eq!(0, res);
    Ok(())
}

#[test]
fn calc_0_returns_0() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[0])?;

    // when invoked
    let res = score(rolls);

    // the score should be 0
    assert_eq!(0, res);
    Ok(())
}

#[test]
fn calc_all_0s_returns_0() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[0; 20])?;

    // when invoked
    let res = score(rolls);

    // the score should be 0
    assert_eq!(0, res);
    Ok(())
}

#[test]
fn calc_1_returns_1() -> Result<()> {
    // given a 1-roll
    let rolls = Rolls::new(&[1])?;

    // when invoked
    let res = score(rolls);

    // the score should be 1
    assert_eq!(1, res);
    Ok(())
}

#[test]
fn calc_all_1s_returns_20() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[1; 20])?;

    // when invoked
    let res = score(rolls);

    // the score should be 20
    assert_eq!(20, res);
    Ok(())
}

#[test]
fn calc_5_5_5_returns_20() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[5; 3])?;

    // when invoked
    let res = score(rolls);

    // the score should be 20
    assert_eq!(20, res);
    Ok(())
}

#[test]
fn calc_all_5s_returns_150() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[5; 21])?;

    // when invoked
    let res = score(rolls);

    // the score should be 150
    assert_eq!(150, res);
    Ok(())
}

#[test]
fn calc_10_10_10_returns_20() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[10; 3])?;

    // when invoked
    let res = score(rolls);

    // the score should be 60
    assert_eq!(60, res);
    Ok(())
}

#[test]
fn calc_all_10s_returns_300() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[10; 22])?;

    // when invoked
    let res = score(rolls);

    // the score should be 300
    assert_eq!(300, res);
    Ok(())
}

