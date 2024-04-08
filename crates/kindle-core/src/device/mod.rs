use crate::tensor::{backend::Backend, Device};

pub trait KindleDevice<B: Backend> {}
impl<B: Backend> KindleDevice<B> for Device<B> {}
