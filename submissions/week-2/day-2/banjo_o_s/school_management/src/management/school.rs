use crate::management::student::{Student, };
use std::collections::HashMap;

pub struct School {
    pub students: HashMap<u128, Student>,
    pub student_key: u128,
}