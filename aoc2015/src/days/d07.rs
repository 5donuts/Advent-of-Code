use std::collections::HashMap;
use utils::PuzzleResult;

// see: https://doc.rust-lang.org/book/appendix-02-operators.html

pub fn part1(input: &str) -> PuzzleResult {
    let c = Circuit::try_from(input).map_err(|e| format!("{:?}", e))?;
    let c = c.eval("a").map_err(|e| format!("{:?}", e))?;
    match &c["a"] {
        Signal::Value(v) => Ok(format!("{}", &v)),
        Signal::Expr(e) => Err(format!("Expected value, got expr: '{:?}'", e)),
    }
}

pub fn part2(input: &str) -> PuzzleResult {
    let mut c = Circuit::try_from(input).map_err(|e| format!("{:?}", e))?;
    let override_signal = c.clone().eval("a").map_err(|e| format!("{:?}", e))?["a"];

    c.0.insert("b", override_signal);
    let c = c.eval("a").map_err(|e| format!("{:?}", e))?;

    match c["a"] {
        Signal::Value(v) => Ok(format!("{}", &v)),
        Signal::Expr(e) => Err(format!("Expected value, got expr: '{:?}'", e)),
    }
}

/// A value on a wire in a [Circuit]
#[derive(Debug, PartialEq, Clone, Copy)]
enum Signal<'a> {
    Value(u16),
    Expr(Expr<'a>),
}

/// A Circuit is a number of wires (identified by one or more letters)
/// that each carry a [Signal]
#[derive(Debug, Clone)]
struct Circuit<'a>(HashMap<&'a str, Signal<'a>>);

impl<'a> Circuit<'a> {
    /// Evaluate this circuit until the [Signal] on the specified wire is known.
    // Use `mut self` instead of `&mut self` b/c passing ownership was easier than
    // managing the lifetimes.
    fn eval(mut self, wire: &'a str) -> Result<Self, Box<dyn std::error::Error>> {
        match &self[wire] {
            // Case 1: The value of the signal on this wire is known.
            Signal::Value(_) => Ok(self),
            // Case 2: The value of the signal on this wire is not yet known.
            Signal::Expr(e) => {
                match e.expr_type {
                    // Case 2a: The value of the signal on this wire comes from another wire.
                    // We must evaluate that wire to determine the signal on this wire.
                    ExprType::Immediate => {
                        let expr_wire = e.l.unwrap();
                        self = self.eval(expr_wire)?;
                        let expr_signal = self[expr_wire];
                        self.0.insert(wire, expr_signal);
                    }
                    // Case 2b: The value of the signal on this wire comes from a gate.
                    // We must evaluate the input to that gate, then the gate itself, to
                    // determine the signal on this wire.
                    ExprType::Unary => {
                        let operator = e.op.unwrap();
                        if operator != Gate::NOT {
                            return Err("'NOT' is the only supported unary operator!".into());
                        }

                        let operand = e.r.unwrap();
                        let operand = if let Ok(val) = operand.parse() {
                            // The operand is an immediate value we hadn't yet parsed
                            Signal::Value(val)
                        } else {
                            // The operand is a wire we must evaluate
                            self = self.eval(operand)?;
                            self[operand]
                        };

                        if let Signal::Value(v) = operand {
                            self.0.insert(wire, Signal::Value(!v));
                        } else {
                            return Err(format!(
                                "Operand should be a value at this point, got: {:?}",
                                operand
                            )
                            .into());
                        }
                    }
                    // Case 2c: The value of the signal on this wire comes from a gate.
                    // We must evaluate the inputs to that gate, then the gate itself, to
                    // determine the signal on this wire.
                    ExprType::Binary => {
                        let l_operand = e.l.unwrap();
                        let r_operand = e.r.unwrap();
                        let operator = e.op.unwrap();

                        let l_operand = if let Ok(val) = l_operand.parse() {
                            Signal::Value(val)
                        } else {
                            self = self.eval(l_operand)?;
                            self[l_operand]
                        };

                        let r_operand = if let Ok(val) = r_operand.parse() {
                            Signal::Value(val)
                        } else {
                            self = self.eval(r_operand)?;
                            self[r_operand]
                        };

                        let (l, r) = match (l_operand, r_operand) {
                            (Signal::Value(l_val), Signal::Value(r_val)) => (l_val, r_val),
                            _ => return Err(format!("Both operands should be values at this point. Got l: {:?}, r: {:?}", l_operand, r_operand).into())
                        };

                        let expr_signal = Signal::Value(match operator {
                            Gate::AND => l & r,
                            Gate::OR => l | r,
                            Gate::LSHIFT => l << r,
                            Gate::RSHIFT => l >> r,
                            Gate::NOT => return Err("'NOT' is not a binary operator!".into()),
                        });

                        self.0.insert(wire, expr_signal);
                    }
                }

                // Once evaluation is done, return this circuit
                Ok(self)
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for Circuit<'a> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut map: HashMap<&str, Signal> = HashMap::new();

        for line in input.lines().map(|l| {
            let mut parts = l.trim().split("->");
            let expr = Expr::try_from(parts.next().unwrap().trim())?;
            let wire = parts.next().unwrap().trim();
            Ok::<(&str, Expr<'_>), Self::Error>((wire, expr))
        }) {
            let (wire, expr) = line?;

            // Only try to evaluate immediate expressions at this point; other expression
            // types will be evaluated after the entire circuit is parsed.
            let signal = match expr.expr_type {
                ExprType::Immediate => {
                    if let Ok(val) = expr.l.unwrap().parse() {
                        Signal::Value(val)
                    } else {
                        Signal::Expr(expr)
                    }
                }
                _ => Signal::Expr(expr),
            };

            map.insert(wire, signal);
        }

        Ok(Self(map))
    }
}

impl<'a> std::ops::Index<&str> for Circuit<'a> {
    type Output = Signal<'a>;

    fn index(&self, index: &str) -> &Self::Output {
        self.0.index(index)
    }
}

/// A single bitwise logic gate in a [Circuit]
#[derive(Debug, PartialEq, Clone, Copy)]
enum Gate {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
}

impl TryFrom<&str> for Gate {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AND" => Ok(Gate::AND),
            "OR" => Ok(Gate::OR),
            "LSHIFT" => Ok(Gate::LSHIFT),
            "RSHIFT" => Ok(Gate::RSHIFT),
            "NOT" => Ok(Gate::NOT),
            _ => Err(format!("No such gate: {}", value)),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ExprType {
    Immediate,
    Unary,
    Binary,
}

/// An expression to evaluate while parsing a [Circuit]
///
/// * For [Immediate](ExprType::Immediate) expressions, there is only a `l` component
/// * For [Unary](ExprType::Unary) expressions, there is an `op` and `r` component
/// * For [Binary](ExprType::Binary) expressions, there are `l`, `op`, and `r` components
#[derive(Debug, PartialEq, Clone, Copy)]
struct Expr<'a> {
    expr_type: ExprType,
    l: Option<&'a str>,
    op: Option<Gate>,
    r: Option<&'a str>,
}

impl<'a> TryFrom<&'a str> for Expr<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let expr_type = {
            let count = value.split(" ").count();
            if count == 1 {
                ExprType::Immediate
            } else if count == 2 {
                ExprType::Unary
            } else {
                ExprType::Binary
            }
        };

        let mut parts = value.split(" ");
        match expr_type {
            ExprType::Immediate => Ok(Self {
                expr_type,
                l: parts.next(),
                op: None,
                r: None,
            }),
            ExprType::Unary => Ok(Self {
                expr_type,
                l: None,
                op: Some(Gate::try_from(parts.next().unwrap())?),
                r: parts.next(),
            }),
            ExprType::Binary => Ok(Self {
                expr_type,
                l: parts.next(),
                op: Some(Gate::try_from(parts.next().unwrap())?),
                r: parts.next(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#;

    #[test]
    fn parse_circuit() {
        let expected = HashMap::from([
            ("x", Signal::Value(123)),
            ("y", Signal::Value(456)),
            ("d", Signal::Expr(Expr::try_from("x AND y").unwrap())),
            ("e", Signal::Expr(Expr::try_from("x OR y").unwrap())),
            ("f", Signal::Expr(Expr::try_from("x LSHIFT 2").unwrap())),
            ("g", Signal::Expr(Expr::try_from("y RSHIFT 2").unwrap())),
            ("h", Signal::Expr(Expr::try_from("NOT x").unwrap())),
            ("i", Signal::Expr(Expr::try_from("NOT y").unwrap())),
        ]);

        assert_eq!(Circuit::try_from(INPUT).unwrap().0, expected);
    }

    #[test]
    fn eval_circuit() {
        use Signal::*;
        let mut circuit = Circuit::try_from(INPUT).unwrap();
        let signals = vec![
            ("d", Value(72)),
            ("e", Value(507)),
            ("f", Value(492)),
            ("g", Value(114)),
            ("h", Value(65412)),
            ("i", Value(65079)),
            ("x", Value(123)),
            ("y", Value(456)),
        ];
        for (wire, expected) in signals {
            circuit = circuit.eval(wire).unwrap();
            assert_eq!(circuit[wire], expected);
        }
    }
}
