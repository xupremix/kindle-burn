use serde::{Deserialize, Serialize};

#[cfg(feature = "ndarray")]
mod ndarray;
#[cfg(feature = "ndarray")]
pub use ndarray::*;

#[cfg(feature = "candle")]
mod candle;
#[cfg(feature = "candle")]
pub use candle::*;

#[cfg(feature = "tch")]
mod tch;
#[cfg(feature = "tch")]
pub use tch::*;

#[cfg(feature = "wgpu")]
mod wgpu;
#[cfg(feature = "wgpu")]
pub use wgpu::*;

#[allow(private_bounds)]
pub trait KindleDevice<'a>:
    crate::Sealed
    + core::fmt::Debug
    + Copy
    + Clone
    + 'static
    + Sized
    + Eq
    + PartialEq
    + Serialize
    + Deserialize<'a>
    + Send
    + Sync
{
}
