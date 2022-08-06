use std::fmt;

#[derive(Clone, Debug)]
pub enum Regex<T> {
    Literal(T),
    Alternative(Box<Regex<T>>, Box<Regex<T>>),
    Concatenation(Box<Regex<T>>, Box<Regex<T>>),
    Star(Box<Regex<T>>),
    Empty,
}

impl<T: Clone> Regex<T> {
    fn precedence(&self) -> i32 {
        match self {
            Regex::Literal(_) => 5,
            Regex::Empty => 5,
            Regex::Alternative(_, _) => 1,
            Regex::Concatenation(_, _) => 2,
            Regex::Star(_) => 3,
        }
    }
    pub fn star(&self) -> Self {
        return Regex::Star(Box::new(self.clone()));
    }
    pub fn concatenate(&self, other: &Self) -> Self {
        return Regex::Concatenation(Box::new(self.clone()), Box::new(other.clone()));
    }
    pub fn alternate(&self, other: &Self) -> Self {
        return Regex::Alternative(Box::new(self.clone()), Box::new(other.clone()));
    }
}

impl<T: fmt::Display + Clone> fmt::Display for Regex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Regex::Literal(c) => f.write_fmt(format_args!("{}", c)),
            Regex::Alternative(a, b) => {
                if a.precedence() >= 1 {
                    f.write_fmt(format_args!("{}", a))?;
                } else {
                    f.write_fmt(format_args!("({})", a))?;
                }
                f.write_fmt(format_args!("|"))?;
                if b.precedence() >= 1 {
                    f.write_fmt(format_args!("{}", b))?;
                } else {
                    f.write_fmt(format_args!("({})", b))?;
                }
                return Ok(());
            }
            Regex::Concatenation(a, b) => {
                if a.precedence() >= 2 {
                    f.write_fmt(format_args!("{}", a))?;
                } else {
                    f.write_fmt(format_args!("({})", a))?;
                }
                if b.precedence() >= 2 {
                    f.write_fmt(format_args!("{}", b))?;
                } else {
                    f.write_fmt(format_args!("({})", b))?;
                }
                return Ok(());
            }
            Regex::Star(a) => {
                if a.precedence() >= 3 {
                    f.write_fmt(format_args!("{}", a))?;
                } else {
                    f.write_fmt(format_args!("({})", a))?;
                }
                f.write_fmt(format_args!("*"))?;
                return Ok(());
            }
            Regex::Empty => f.write_fmt(format_args!("ε")),
        }
    }
}
