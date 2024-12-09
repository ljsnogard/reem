use core::{
    borrow::Borrow,
    marker::PhantomData,
    ops::Range,
};

use abs_reem::{TrAbility, TrCandidateId, TrExperience, TrResume};
use super::{common::RoundId, position::Position};

pub struct Experience<C, P, A>
where
    C: TrCandidateId,
    P: Borrow<Position<A>>,
    A: TrAbility,
{
    candidate_id_: C,
    position_ref_: P,
    period_: Range<RoundId>,
    _unused_a_: PhantomData<Position<A>>,
}

impl<C, P, A> Experience<C, P, A>
where
    C: TrCandidateId,
    P: Borrow<Position<A>>,
    A: TrAbility,
{
    pub const fn new(
        candidate_id: C,
        position: P,
        period: Range<RoundId>,
    ) -> Self {
        Experience {
            candidate_id_: candidate_id,
            position_ref_: position,
            period_: period,
            _unused_a_: PhantomData,
        }
    }

    pub const fn candidate_id(&self) -> &C {
        &self.candidate_id_
    }

    pub fn position(&self) -> &Position<A> {
        self.position_ref_.borrow()
    }

    pub fn period(&self) -> &Range<RoundId> {
        &self.period_
    }

    pub fn requirement(&self) -> &A {
        self.position().requirement()
    }
}

impl<C, P, A> TrExperience for Experience<C, P, A>
where
    C: TrCandidateId,
    P: Borrow<Position<A>>,
    A: TrAbility,
{
    type Ability = A;
    type CandidateId = C;
    type RoundId = RoundId;

    #[inline]
    fn candidate_id(&self) -> &Self::CandidateId {
        Experience::candidate_id(self)
    }

    #[inline]
    fn period(&self) -> &Range<Self::RoundId> {
        Experience::period(self)
    }

    #[inline]
    fn requirement(&self) -> &Self::Ability {
        Experience::requirement(self)
    }
}

pub struct Resume<C, B, E>
where
    C: TrCandidateId,
    B: Borrow<[E]>,
    E: TrExperience<CandidateId = C>,
{
    candidate_id_: C,
    experiences_: B,
    _phantom_e_: PhantomData<[E]>,
}

impl<C, B, E> Resume<C, B, E>
where
    C: TrCandidateId,
    B: Borrow<[E]>,
    E: TrExperience<CandidateId = C>,
{
    pub const fn new(
        candidate_id: C,
        experiences: B,
    ) -> Self {
        Resume {
            candidate_id_: candidate_id,
            experiences_: experiences,
            _phantom_e_: PhantomData,
        }
    }

    pub const fn candidate_id(&self) -> &C {
        &self.candidate_id_
    }

    pub fn experiences(&self) -> impl IntoIterator<Item = &E> {
        self.experiences_.borrow().iter()
    }
}

impl<C, B, E> TrResume for Resume<C, B, E>
where
    C: TrCandidateId,
    B: Borrow<[E]>,
    E: TrExperience<CandidateId = C>,
{
    type Ability = <E as TrExperience>::Ability;
    type CandidateId = C;
    type Experience = E;

    #[inline]
    fn candidate_id(&self) -> &Self::CandidateId {
        Resume::candidate_id(self)
    }

    #[inline]
    fn experiences(&self) -> impl IntoIterator<Item = &Self::Experience> {
        Resume::experiences(self)
    }
}
