use super::Solution;

/// A direction of mass flow
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum Direction {
    Positive,
    Negative,
    Unknown,
}

impl Direction {
    /// Return a `Direction` based on the sign of a number
    ///
    /// If `value` is exactly `0.0`, a positive direction is assumed.
    pub(super) fn from_value(value: f64) -> Self {
        if value >= 0.0 {
            Self::Positive
        } else {
            Self::Negative
        }
    }

    /// Return a value based on the direction of `self`
    ///
    /// An average of the two values is returned if the direction is `Unknown`.
    pub(super) fn select(self, positive: f64, negative: f64) -> f64 {
        match self {
            Self::Positive => positive,
            Self::Negative => negative,
            Self::Unknown => 0.5 * (positive + negative),
        }
    }
}

/// The direction of mass flow between each control volume
///
/// The abbreviations are:
///   - `ck` compression space to cold heat exchanger
///   - `kr` cold heat exchanger to regenerator
///   - `rl` regenerator to hot heat exchanger
///   - `le` hot heat exchanger to expansion space
///
/// Positive flow is from the first volume in the abbreviation to the second.
/// For example, a `Direction::Positive` for `kr` means mass is flowing from
/// the cold heat exhanger into the regenerator.  A `Direction::Negative` for
/// `le` means flow is from the expansion space to the hot heat exhanger.
#[derive(Clone, Copy, PartialEq)]
pub(super) struct FlowDirection {
    pub(super) ck: Direction,
    pub(super) kr: Direction,
    pub(super) rl: Direction,
    pub(super) le: Direction,
}

impl FlowDirection {
    /// Determine the flow directions from a `Solution`
    pub(super) fn from_solution(solution: &Solution) -> Self {
        Self {
            ck: Direction::from_value(solution.m_dot_ck),
            kr: Direction::from_value(solution.m_dot_kr),
            rl: Direction::from_value(solution.m_dot_rl),
            le: Direction::from_value(solution.m_dot_le),
        }
    }
}

impl Default for FlowDirection {
    fn default() -> Self {
        Self {
            ck: Direction::Unknown,
            kr: Direction::Unknown,
            rl: Direction::Unknown,
            le: Direction::Unknown,
        }
    }
}
