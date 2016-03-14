pub trait RuleName: 'static + Copy + Eq + Ord {}

impl<T> RuleName for T where T: 'static + Copy + Eq + Ord {}
