use std::marker::PhantomData;

use crate::{Float, From};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3<SPACE = (), WHITE = ()>(Float, Float, Float, PhantomData<(SPACE, WHITE)>);

impl<S, W> Vec3<S, W> {
    #[allow(clippy::just_underscores_and_digits)]
    pub const fn new(_0: Float, _1: Float, _2: Float) -> Self {
        Vec3(_0, _1, _2, PhantomData)
    }

    pub const fn tuple(self) -> (Float, Float, Float) {
        (self.0, self.1, self.2)
    }

    pub fn into<SPACE, WHITE>(self) -> Vec3<SPACE, WHITE>
    where
        Vec3<SPACE, WHITE>: From<S, W>,
    {
        From::from(self)
    }

    pub fn ref_0(&self) -> &Float {
        &self.0
    }

    pub fn ref_1(&self) -> &Float {
        &self.1
    }

    pub fn ref_2(&self) -> &Float {
        &self.2
    }

    pub fn mut_0(&mut self) -> &mut Float {
        &mut self.0
    }

    pub fn mut_1(&mut self) -> &mut Float {
        &mut self.1
    }

    pub fn mut_2(&mut self) -> &mut Float {
        &mut self.2
    }
}
