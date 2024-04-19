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
}