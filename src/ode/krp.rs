//! krp
//!

use crate::ode::*;

/// krp
#[derive(Clone, Copy)]
pub struct Krp {
  /// kinetics (false: keep position)
  pub k: bool,
  /// gravity sensitive (false: call dBodyDisable())
  pub g: bool,
  /// collision
  pub c: bool,
  /// bounce
  pub bounce: dReal,
  /// mu (e.g. polyethylene 0.06 to 0.3) default 0.3
  pub mu: dReal
}

impl Krp {
  /// construct
  pub fn new(k: bool, g: bool, c: bool, bounce: dReal, mu: dReal) -> Self {
    Krp{k, g, c, bounce, mu}
  }
}

/// private const F: false
const F: bool = false;
/// private const T: true
const T: bool = true;

/// static krp n k
pub static KRPnk: Krp = Krp{k: F, g: F, c: F, bounce: 0.0001, mu: 0.3};
/// static krp 1.0
pub static KRP100: Krp = Krp{k: T, g: T, c: T, bounce: 1.0, mu: 0.3};
/// static krp 0.95
pub static KRP095: Krp = Krp{k: T, g: T, c: T, bounce: 0.95, mu: 0.3};
/// static krp 0.9
pub static KRP090: Krp = Krp{k: T, g: T, c: T, bounce: 0.9, mu: 0.3};
/// static krp 0.8
pub static KRP080: Krp = Krp{k: T, g: T, c: T, bounce: 0.8, mu: 0.3};
/// static krp 0.7
pub static KRP070: Krp = Krp{k: T, g: T, c: T, bounce: 0.7, mu: 0.3};
/// static krp 0.6
pub static KRP060: Krp = Krp{k: T, g: T, c: T, bounce: 0.6, mu: 0.3};
/// static krp 0.5
pub static KRP050: Krp = Krp{k: T, g: T, c: T, bounce: 0.5, mu: 0.3};
/// static krp 0.4
pub static KRP040: Krp = Krp{k: T, g: T, c: T, bounce: 0.4, mu: 0.3};
/// static krp 0.3
pub static KRP030: Krp = Krp{k: T, g: T, c: T, bounce: 0.3, mu: 0.3};
/// static krp 0.2
pub static KRP020: Krp = Krp{k: T, g: T, c: T, bounce: 0.2, mu: 0.3};
/// static krp 0.1
pub static KRP010: Krp = Krp{k: T, g: T, c: T, bounce: 0.1, mu: 0.3};
/// static krp 0.01
pub static KRP001: Krp = Krp{k: T, g: T, c: T, bounce: 0.01, mu: 0.3};
