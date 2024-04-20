use std::{cell::RefCell, rc::Rc};

use crate::{OperatorEnum, Tensor};

pub struct Record {
    pub operator: Rc<RefCell<OperatorEnum>>,
    pub inputs: Vec<Tensor>,
    pub output: Tensor,
}

pub struct Tape {
    pub records: Vec<Record>,
}

impl Default for Tape {
    fn default() -> Self {
        Self {
            records: Default::default(),
        }
    }
}

impl Tape {
    pub fn push(
        &mut self,
        operator: Rc<RefCell<OperatorEnum>>,
        inputs: Vec<Tensor>,
        output: Tensor,
    ) {
        self.records.push(Record {
            operator,
            inputs,
            output,
        })
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }

    pub fn print_records(&self) {
        println!("Tape records: {}", self.records.len());
        for record in self.records.iter() {
            let operator = &record.operator;
            let inputs = &record.inputs;

            let operator_name = (*operator).borrow().name();
            println!(
                "Tape is recording a record: operator: {}  inputs: {}  output: {}",
                operator_name,
                inputs.len(),
                1
            );
        }
    }
}
