use core::ops::Range;

/// N 维矢量，用于描述职位要求，候选人能力
pub trait TrAbility
where
    Self: Clone + Copy,
{
    type NumType: num_traits::Num;
    const N: usize;

    fn vector(&self) -> &[Self::NumType; Self::N];
}

pub trait TrRoundId
where
    Self: Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Into<usize>,
{}

pub trait TrCandidateId
where
    Self: Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Into<usize>,
{}

pub trait TrPositionId
where
    Self: Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Into<usize>,
{}

/// A record item in the resume of a candidate, which includes the position
/// requirement, the ability
/// 
/// 单条工作经历记录，包括当时的岗位要求，候选人当时的能力数据，以及履职时长（轮数）
pub trait TrExperience {
    type Ability: TrAbility;
    type RoundId: TrRoundId;

    /// 该工作经历的技能要求
    fn requirement(&self) -> &Self::Ability;

    fn period(&self) -> Range<Self::RoundId>;
}

pub trait TrResume {
    type Ability: TrAbility;
    type CandidateId: TrCandidateId;
    type Experience: TrExperience<Ability = Self::Ability>;

    fn candidate_id(&self) -> Self::CandidateId;

    fn experiences(&self) -> impl IntoIterator<Item = &Self::Experience>;
}

/// 输入岗位技能要求，和多个候选人技能数据，输出匹配的候选人并按匹配程度降序排列
pub trait TrCandidateSelect {
    type Ability: TrAbility;
    type CandidateId: TrCandidateId;
    type RoundId: TrRoundId;

    /// Returns Some(round) if the capability meets the requirement, else None.
    fn sort<'a>(
        &'a self,
        current_round: Self::RoundId,
        requirement: &'a Self::Ability,
        get_capability: impl Fn(Self::CandidateId) -> &'a Self::Ability,
        id_iterator: impl IntoIterator<Item = Self::CandidateId>,
    ) -> impl IntoIterator<Item = (Self::CandidateId, Range<Self::RoundId>)>;
}

/// 输入候选人技能数据，和多个岗位 Offer 的技能要求，输出 Offer 按偏好程度降序排列
pub trait TrOfferSelect {
    type Ability: TrAbility;
    type PositionId: TrPositionId;
    type RoundId: TrRoundId;

    fn sort<'a>(
        &'a self,
        current_round: Self::RoundId,
        capability: &'a Self::Ability,
        get_requirement: impl Fn(Self::PositionId) -> &'a Self::Ability,
        id_iterator: impl IntoIterator<Item = Self::PositionId>,
    ) -> impl IntoIterator<Item = Self::PositionId>;
}

/// 通过履历和岗位技能要求，计算候选人匹配程度，并根据匹配程度降序输出
pub trait TrResumeSelect {
    type Ability: TrAbility;
    type Resume: TrResume<Ability = Self::Ability>;
    type RoundId: TrRoundId;

    /// Returns true if the resume meets the requirement and the candidates
    /// should go further for qualification test.
    fn sort<'a>(
        &'a self,
        current_round: Self::RoundId,
        requirement: &'a Self::Ability,
        resume_iter: impl IntoIterator<Item = &'a Self::Resume>,
    ) -> impl IntoIterator<Item = <Self::Resume as TrResume>::CandidateId>;
}

/// 输入入职期限，岗位要求，和候选人技能，计算任职后的候选人技能
pub trait TrEmploymentTransform {
    type Ability: TrAbility;
    type RoundId: TrRoundId;

    /// 输入入职期限，岗位要求，和候选人技能，计算任职后的候选人技能
    /// 
    /// ## Arguments
    /// - `start`: 入职轮次
    /// - `end`: 离职轮次
    /// - `requirement`: 岗位技能要求
    /// - `capability`: 入职时的候选人技能
    fn employment_transform(
        &self,
        start: Self::RoundId,
        end: Self::RoundId,
        requirement: &Self::Ability,
        capability: &Self::Ability,
    ) -> Self::Ability;
}
