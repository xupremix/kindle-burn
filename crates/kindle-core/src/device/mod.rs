use serde::{Deserialize, Serialize};

#[cfg(feature = "ndarray")]
mod ndarray;
#[cfg(feature = "ndarray")]
pub use ndarray::*;
#[cfg(all(feature = "ndarray", feature = "autodiff"))]
mod ndarray_autodiff;

#[cfg(feature = "candle")]
mod candle;
#[cfg(feature = "candle")]
pub use candle::*;
#[cfg(all(feature = "candle", feature = "autodiff"))]
mod candle_autodiff;

#[cfg(feature = "tch")]
mod tch;
#[cfg(feature = "tch")]
pub use tch::*;
#[cfg(all(feature = "tch", feature = "autodiff"))]
mod tch_autodiff;

#[cfg(feature = "wgpu")]
mod wgpu;
#[cfg(feature = "wgpu")]
pub use wgpu::*;
#[cfg(all(feature = "wgpu", feature = "autodiff"))]
mod wgpu_autodiff;

#[cfg(feature = "fusion")]
mod fusion;
#[cfg(feature = "fusion")]
pub use fusion::*;
#[cfg(all(feature = "fusion", feature = "autodiff"))]
mod fusion_autodiff;

pub trait KindleDevice<Backend>:
    core::fmt::Debug + Clone + 'static + Sized + Serialize + Deserialize<'static> + Send + Sync
where
    Backend: crate::tensor::backend::Backend,
{
    fn to_device() -> Backend::Device;
}
