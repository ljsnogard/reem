use core::{
    borrow::BorrowMut,
    convert::AsRef,
    future::IntoFuture,
    num::NonZeroUsize,
};

use futures_lite::stream::Stream;

pub trait TrRoundNumber
where
    Self: Clone + Copy + Into<NonZeroUsize>
{}

/// N 维矢量，用于描述职位要求，候选人能力
pub trait TrAbility
where
    Self: BorrowMut<[Self::NumType]>
{
    type NumType: num_traits::Num;
    const N: usize;

    fn vector(&self) -> &[Self::NumType; Self::N];
}

/// A record item in the resume of a candidate, which includes the position
/// requirement, the ability
/// 
/// 工作履历记录，包括当时的岗位要求，候选人当时的能力数据，以及履职时长（轮数）
pub trait TrRecord {
    type Ability: TrAbility;

    fn requirement(&self) -> &Self::Ability;

    fn duration(&self) -> impl TrRoundNumber;
}

pub trait TrResume {
    type Ability: TrAbility;
    type Record: TrRecord<Ability = Self::Ability>;

    fn records(&mut self) -> impl Stream<Item = Self::Record>;
}

pub trait TrCandidateTest {
    type Ability: TrAbility;
    type Duration: TrRoundNumber;

    /// Returns Some(round) if the capability meets the requirement, else None.
    fn test_async(
        &mut self,
        requirement: &Self::Ability,
        candidate: &mut impl AsRef<Self::Ability>,
    ) -> impl IntoFuture<Output = Option<Self::Duration>>;
}

pub trait TrResumeTest {
    type Ability: TrAbility;

    type Resume: TrResume<Ability = Self::Ability>;
    type Duration: TrRoundNumber;

    /// Returns true if the resume meets the requirement and the candidates
    /// should go further for qualification test.
    fn test_async(
        &mut self,
        requirement: &Self::Ability,
        resume: &mut Self::Resume,
    ) -> impl IntoFuture<Output = bool>;
}
