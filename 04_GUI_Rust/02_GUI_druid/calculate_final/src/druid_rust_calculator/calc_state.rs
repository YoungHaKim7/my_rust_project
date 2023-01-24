// External Crates
use druid::{Data, Lens};
use rug::{float::Round, ops::Pow, Float};

// Use druid_float from module
use super::druid_float::DruidFloat;

#[derive(Clone, Data, Lens)]
// Used with direct access to fields in mod.rs and main.rs, so must be public
pub struct CalcState {
    /// The number displayed. Generally a valid float.
    pub value: String,
    pub operand: DruidFloat,
    pub operator: char,
    pub in_num: bool,
}

// Functions used within mod.rs, so must be public
impl CalcState {
    pub fn digit(&mut self, digit: u8) {
        if !self.in_num {
            self.value.clear();
            self.in_num = true;
        }
        let ch = (b'0' + digit) as char;
        self.value.push(ch);
    }

    pub fn display(&mut self) {
        // TODO: change hyphen-minus to actual minus
        self.value = self.operand.value.to_string();
    }

    pub fn compute(&mut self) {
        if self.in_num {
            let temp = self.value.parse().unwrap_or(0.0);
            let operand2 = DruidFloat {
                value: Float::with_val(16, temp),
            };
            let result = match self.operator {
                '+' => Some(self.operand.value.to_owned() + operand2.value),
                '−' => Some(self.operand.value.to_owned() - operand2.value),
                '×' => Some(self.operand.value.to_owned() * operand2.value),
                '÷' => Some(self.operand.value.to_owned() / operand2.value),
                '%' => Some(self.operand.value.to_owned() % operand2.value),
                '«' => Some(Float::with_val(
                    16,
                    self.operand
                        .value
                        .to_owned()
                        .to_i32_saturating_round(Round::Up)
                        .unwrap()
                        << operand2.value.to_i32_saturating_round(Round::Up).unwrap(),
                )),
                '»' => Some(Float::with_val(
                    16,
                    self.operand
                        .value
                        .to_owned()
                        .to_i32_saturating_round(Round::Up)
                        .unwrap()
                        >> operand2.value.to_i32_saturating_round(Round::Up).unwrap(),
                )),
                '^' => Some(self.operand.value.to_owned().pow(operand2.value)),
                _ => None,
            };
            if let Some(result) = result {
                self.operand.value = Float::from(result);
                self.display();
                self.in_num = false;
            }
        }
    }

    pub fn op(&mut self, op: char) {
        match op {
            '+' | '−' | '×' | '÷' | '=' | '%' | '^' | '»' | '«' => {
                self.compute();
                let temp = self.value.parse().unwrap_or(0.0);
                self.operand = DruidFloat {
                    value: Float::with_val(16, temp),
                };
                self.operator = op;
                self.in_num = false;
            }
            '±' => {
                if self.in_num {
                    if self.value.starts_with('−') {
                        self.value = self.value[3..].to_string();
                    } else {
                        self.value = ["−", &self.value].concat();
                    }
                } else {
                    self.operand.value = -self.operand.value.to_owned();
                    self.display();
                }
            }
            '.' => {
                if !self.in_num {
                    self.value = "0".to_string();
                    self.in_num = true;
                }
                if self.value.find('.').is_none() {
                    self.value.push('.');
                }
            }
            'c' => {
                self.value = "0".to_string();
                self.in_num = false;
            }
            'C' => {
                self.value = "0".to_string();
                self.operator = 'C';
                self.in_num = false;
            }
            'D' => {
                if self.in_num {
                    self.value.pop();
                    if self.value.is_empty() || self.value == "−" {
                        self.value = "0".to_string();
                        self.in_num = false;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
