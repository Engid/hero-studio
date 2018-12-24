use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::SampleRate;

pub const NANOS_PER_SECOND: u64 = 1_000_000_000;
pub const PICOS_PER_SECOND: u64 = 1_000_000_000_000;
pub const FEMTOS_PER_SECOND: u64 = 1_000_000_000_000_000;
pub const ATTOS_PER_SECOND: u64 = 1_000_000_000_000_000_000;

pub type UnitType = u64;
pub const UNITS_PER_SECOND: UnitType = PICOS_PER_SECOND as UnitType;

///! High resolution time
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct ClockTime(UnitType);

impl ClockTime {
  pub fn zero() -> ClockTime {
    ClockTime(0)
  }

  pub fn new(units: UnitType) -> ClockTime {
    ClockTime(units)
  }

  pub fn from_seconds(seconds: f64) -> ClockTime {
    ClockTime((seconds * UNITS_PER_SECOND as f64).round() as UnitType)
  }

  pub fn from_samples(samples: u32, sample_rate: SampleRate) -> ClockTime {
    ClockTime(samples as UnitType * UNITS_PER_SECOND / sample_rate as UnitType)
  }

  pub fn error_per_sample(sample_rate: SampleRate) -> f64 {
    let time_per_sample = ClockTime::from_samples(1, sample_rate);
    let error_per_second = ClockTime::from_seconds(1.0) - (time_per_sample * sample_rate);
    error_per_second.0 as f64 / sample_rate as f64
  }

  pub fn units(&self) -> UnitType {
    self.0
  }
}

impl Add for ClockTime {
  type Output = ClockTime;

  fn add(self, rhs: ClockTime) -> ClockTime {
    ClockTime(self.0 + rhs.0)
  }
}

impl AddAssign for ClockTime {
  fn add_assign(&mut self, rhs: ClockTime) {
    *self = *self + rhs;
  }
}

impl Sub for ClockTime {
  type Output = ClockTime;

  fn sub(self, rhs: ClockTime) -> ClockTime {
    ClockTime(self.0 - rhs.0)
  }
}

impl SubAssign for ClockTime {
  fn sub_assign(&mut self, rhs: ClockTime) {
    *self = *self - rhs;
  }
}

impl Mul<u32> for ClockTime {
  type Output = ClockTime;

  fn mul(self, rhs: u32) -> ClockTime {
    ClockTime(self.0 * rhs as UnitType)
  }
}

impl MulAssign<u32> for ClockTime {
  fn mul_assign(&mut self, rhs: u32) {
    *self = *self * rhs;
  }
}

impl Mul<ClockTime> for u32 {
  type Output = ClockTime;

  fn mul(self, rhs: ClockTime) -> ClockTime {
    rhs * self
  }
}

impl Div<u32> for ClockTime {
  type Output = ClockTime;

  fn div(self, rhs: u32) -> ClockTime {
    ClockTime(self.0 / rhs as UnitType)
  }
}

impl DivAssign<u32> for ClockTime {
  fn div_assign(&mut self, rhs: u32) {
    *self = *self / rhs;
  }
}

#[cfg(test)]
mod test {
  use super::ClockTime;

  #[test]
  pub fn clock_time_new() {
    let time = ClockTime::new(15);
    assert_eq!(time.units(), 15);
  }

  #[test]
  pub fn clock_time_zero() {
    let time = ClockTime::zero();
    assert_eq!(time.units(), 0);
  }

  #[test]
  pub fn clock_time_add() {
    let time1 = ClockTime::new(15);
    let time2 = ClockTime::new(5);
    assert_eq!(time1 + time2, ClockTime::new(20));
  }

  #[test]
  pub fn clock_time_add_assign() {
    let mut time1 = ClockTime::new(15);
    time1 += ClockTime::new(5);
    assert_eq!(time1, ClockTime::new(20));
  }

  #[test]
  pub fn clock_time_sub() {
    let time1 = ClockTime::new(15);
    let time2 = ClockTime::new(5);
    assert_eq!(time1 - time2, ClockTime::new(10));
  }

  #[test]
  pub fn clock_time_sub_assign() {
    let mut time1 = ClockTime::new(15);
    time1 -= ClockTime::new(5);
    assert_eq!(time1, ClockTime::new(10));
  }

  #[test]
  pub fn clock_time_mul() {
    let time1 = ClockTime::new(15);
    assert_eq!(time1 * 2u32, ClockTime::new(30));
  }

  #[test]
  pub fn clock_time_mul_assign() {
    let mut time1 = ClockTime::new(15);
    time1 *= 2u32;
    assert_eq!(time1, ClockTime::new(30));
  }

  #[test]
  pub fn clock_time_mul_rhs() {
    let time1 = ClockTime::new(15);
    assert_eq!(2u32 * time1, ClockTime::new(30));
  }

  #[test]
  pub fn clock_time_div() {
    let time1 = ClockTime::new(30);
    assert_eq!(time1 / 2u32, ClockTime::new(15));
  }

  #[test]
  pub fn clock_time_div_assign() {
    let mut time1 = ClockTime::new(30);
    time1 /= 2u32;
    assert_eq!(time1, ClockTime::new(15));
  }
}
