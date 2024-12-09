use abs_reem::{TrAbility, TrPositionId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PositionId(usize);

impl From<PositionId> for usize {
    fn from(value: PositionId) -> Self {
        value.0
    }
}

impl TrPositionId for PositionId
{}

pub struct Position<A>
where
    A: TrAbility
{
    id_: PositionId,
    rq_: A,
}

impl<A> Position<A>
where
    A: TrAbility,
{
    pub const fn new(id: PositionId, requirement: A) -> Self {
        Position {
            id_: id,
            rq_: requirement,
        }
    }

    pub const fn id(&self) -> &PositionId {
        &self.id_
    }

    pub const fn requirement(&self) -> &A {
        &self.rq_
    }
}