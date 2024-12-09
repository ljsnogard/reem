use core::{
    borrow::Borrow,
    marker::PhantomData,
};

use abs_reem::{TrAbility, TrCandidateId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CandidateId(usize);

impl From<CandidateId> for usize {
    fn from(value: CandidateId) -> Self {
        value.0
    }
}

impl TrCandidateId for CandidateId
{}

#[derive(Debug)]
pub struct Candidate<B, A>
where
    B: Borrow<A>,
    A: TrAbility,
{
    _a_: PhantomData<A>,
    id_: CandidateId,
    ab_: B,
}

impl<B, A> Candidate<B, A>
where
    B: Borrow<A>,
    A: TrAbility,
{
    pub const fn new(id: CandidateId, ability: B) -> Self {
        Candidate {
            _a_: PhantomData,
            id_: id,
            ab_: ability,
        }
    }

    pub const fn id(&self) -> &CandidateId {
        &self.id_
    }

    pub fn ability(&self) -> &A {
        self.ab_.borrow()
    }
}
