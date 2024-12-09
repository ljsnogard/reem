use core::{
    borrow::Borrow,
    marker::PhantomData,
};

use abs_reem::{
    x_deps::num_traits,
    TrAbility, TrRoundId,
};

#[derive(Clone)]
pub struct Ability<T, const N: usize, B>(B, PhantomData<[T; N]>)
where
    T: Copy + num_traits::Num,
    B: Borrow<[T; N]> + Clone;

impl<T, const N: usize, B> TrAbility for Ability<T, N, B>
where
    T: Copy + num_traits::Num,
    B: Borrow<[T; N]> + Clone,
{
    type NumType = T;

    fn vector(&self) -> &[Self::NumType] {
        self.0.borrow()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RoundId(usize);

impl From<RoundId> for usize {
    fn from(value: RoundId) -> Self {
        value.0
    }
}

impl TrRoundId for RoundId
{}
