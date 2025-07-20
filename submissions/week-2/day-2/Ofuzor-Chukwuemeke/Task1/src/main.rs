#[derive(Clone, Debug)]
enum Grade {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Clone, Debug)]
enum Status {
    ACTIVE,
    INACTIVE,
}

#[derive(Debug, Clone)]
struct Class {
    name: String,
    grade: Grade,
    status: Status,
}

#[derive(Debug, Clone)]
struct ClassList {
    class: Vec<Class>,
}

impl ClassList {
    pub fn initialize() -> Self {
        Self { class: Vec::new() }
    }

    pub fn create_class(&mut self, class: Class) {
        self.class.push(class);
    }

    pub fn get_classs(self) -> Vec<Class> {
        self.class
    }

    pub fn get_class(self, index: usize) {
        let find_class = self.class.get(index).unwrap();
        // find_class;
        println!("{:?}", find_class)
    }

    pub fn edit_class(&mut self, index: usize) {
        // let class = self.class.
    }
}

fn main() {
    let mut new_class = ClassList::initialize();
    let class1 = Class {
        name: "Emeke".to_string(),
        grade: Grade::A,
        status: Status::ACTIVE,
    };
    let class2 = Class {
        name: "Ofuzor".to_string(),
        grade: Grade::B,
        status: Status::INACTIVE,
    };
    new_class.create_class(class1);
    new_class.create_class(class2);
    // let one_class = &new_class.clone().get_class(1);
    let all_class = new_class.get_classs();
    println!("{:?}", all_class);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initialize() {
        let class = Class {
            name: "Emeke".to_string(),
            grade: Grade::A,
            status: Status::ACTIVE,
        };
        let class = ClassList::initialize();
        assert_eq!(class.class.len(), 0);
    }

    #[test]
    fn test_create() {
        let class1 = Class {
            name: "Emeke".to_string(),
            grade: Grade::A,
            status: Status::ACTIVE,
        };
        let mut class = ClassList::initialize();
        class.create_class(class1);
        assert!(class.class.len() == 1);
    }
}
