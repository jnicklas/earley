pub trait Lexeme: 'static + Copy + Eq + Ord {}

impl<T> Lexeme for T where T: 'static + Copy + Eq + Ord {}
