use std::collections::HashMap;
#[allow(unused_imports)]
use std::default;

use gen_parser::{For, PropsKey, Value};
use gen_utils::common::Ulid;

#[derive(Clone, Debug, Default)]
pub enum Role {
    If {
        id: Ulid,
        props: HashMap<PropsKey, Value>,
    },
    For {
        id: Ulid,
        credential: For,
        loop_type: String,
        props: HashMap<PropsKey, Value>,
    },
    #[default]
    Normal,
}

impl Role {
    pub fn new_if(props: HashMap<PropsKey, Value>) -> Self {
        Role::If {
            id: Ulid::new(),
            props,
        }
    }
    pub fn new_for(credential: For, loop_type: String, props: HashMap<PropsKey, Value>) -> Self {
        Role::For {
            id: Ulid::new(),
            props,
            credential,
            loop_type,
        }
    }
    pub fn is_special(&self) -> bool {
        !matches!(self, Role::Normal)
    }
}
