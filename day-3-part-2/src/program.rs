use tracing::{debug, trace};

mod parser;

#[derive(Clone, Debug)]
pub(super) enum Expression {
    Mul(i32, i32),
    Enable,
    Disable,
}

pub(super) struct Program {
    expressions: Vec<Expression>,
    enabled: bool,
}

impl Program {
    pub(super) fn new(instructions: Vec<Expression>) -> Self {
        Self {
            expressions: instructions,
            enabled: true,
        }
    }

    pub(super) fn parse(s: &str) -> Result<Self, String> {
        parser::parse(s)
            .map(|(_, instructions)| {
                trace!(?instructions, "parsed instructions");
                Self::new(instructions)
            })
            .map_err(|e| e.to_string())
    }

    pub(super) fn eval(&mut self) -> Option<i32> {
        // default the output to a sum for the sake of the day's problem
        let mut sum = 0;
        for instruction in &self.expressions {
            match instruction {
                Expression::Mul(a, b) => {
                    let result = a * b;
                    debug!(enabled = self.enabled, "mul({}, {}) = {}", a, b, result);
                    if self.enabled {
                        sum += result;
                    }
                }
                Expression::Enable => {
                    debug!(enabled = self.enabled, "enable");
                    self.enabled = true
                }
                Expression::Disable => {
                    debug!(enabled = self.enabled, "disable");
                    self.enabled = false
                }
            }
        }
        Some(sum)
    }
}
