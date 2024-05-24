//! ode integrates bindings to cppbridge and cppode
//!
//! - cc-rs https://crates.io/crates/cc
//! - bindgen https://crates.io/crates/bindgen
//!
//! # cc-rs
//!
//! - include/bridge.hpp
//! - src/bridge.cpp
//!
//! # bindgen
//!
//! from
//!
//!  - include/bridge.hpp
//!  - ode/ode.hpp (from modified preprocess -E dum.cpp includes ode.h)
//!
//! to
//!
//!  - include/bridge_bindings.rs
//!  - ode/ode_bindings.rs
//!
//! # Requirements
//!
//! in the running directory
//!
//! - ode.dll
//! - libstdc++-6.dll
//! - libgcc_s_seh-1.dll
//! - libwinpthread-1.dll
//!

#![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod cppbridge;
use cppbridge::*;
pub use cppbridge::{dMULTIPLY0_331, dMULTIPLY0_333};
pub use cppbridge::{dMULTIPLY0_441, dMULTIPLY0_444};
pub use cppbridge::{convexfvp, trimeshvi};
pub use cppbridge::{CopyConvexFVP, CopyTriMeshVI};
// use cppbridge::{ScaleConvexFVP, ScaleTriMeshVI, SetScaleLimit}; // private
pub use cppbridge::{CvtConvexFVPFromTriMeshVI, CvtTriMeshVIFromConvexFVP};
pub use cppbridge::{FreeConvexFVP, FreeTriMeshVI};
pub use cppbridge::{RecalcFaces, Normal4, Cross3};

/// replace bridge.hpp (defined in ode.hpp)
pub type dReal = f64;
/// replace bridge.hpp (defined in ode.hpp)
pub type dTriIndex = u32;

mod cppode;
pub use cppode::*;
/*
pub use cppode::{dBodyID, dGeomID, dTriIndex};
pub use cppode::{dMatrix4, dMatrix3, dVector4, dVector3, dReal}; // 16 12 4 4
pub use cppode::{dQuaternion};
*/

#[warn(unused)]
// #[warn(unused_imports)]
// #[warn(unused_attributes)]
#[warn(non_snake_case)]
#[warn(non_camel_case_types)]
#[warn(non_upper_case_globals)]

pub mod err;
use err::*;

pub mod mat;
use mat::*;

pub mod prim;
use prim::*;
pub use prim::{Matrix4, M4I, Matrix3, M3I, Quaternion, QI};
pub use prim::{PId, PI, PIh, PIt, PIq, PIx};
pub use prim::{PIh3, PIt2, PIt4, PIt5, PIq3, PIx5};

pub mod krp;
use krp::*;
pub use krp::{Krp, KRPnk, KRP100, KRP095, KRP090, KRP080, KRP070};
pub use krp::{KRP060, KRP050, KRP040, KRP030, KRP020, KRP010, KRP001};

/// size_type_of
pub fn size_type_of<T>(_t: &T) -> (usize, &str) {
  (std::mem::size_of::<T>(), std::any::type_name::<T>()) // or type_name_of_val
}
