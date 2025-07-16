#[cfg(test)]
mod tests {
    use crate::{school::School, student::Student, student::StudentStatus};

    #[test]
    fn test_create_student() {
        let student_name = String::from("name");
        let std_grade: u8 = 40;

        let mut school = School::new();

        // let std = Student::new(name, grade);
        school.add_student(student_name, std_grade);

        assert_eq!(school.student_key, 1);
    }

    #[test]
    // #[should_panic(expected= "invalid key provided")]
    fn remove_student() {
        let student_name = String::from("name");
        let std_grade: u8 = 40;

        let mut school: School = School::new();

        school.add_student(student_name, std_grade);

        // let result = school.remove_student(1);

        assert_eq!(school.student_key, 1);

        let student = school.get_student(1);
        assert_eq!(student.unwrap().grade, 40);

        println!("{}", school.student_key);

        //remove student
        school.remove_student(1);

        let student: Option<&Student> = school.get_student(1);

        assert_eq!(student.is_none(), true);

        // assert_eq!(student_option.is_none(), true);
    }

    #[test]
    fn update_student_status() {
        let student_name = String::from("name");
        let std_grade: u8 = 40;

        let mut school: School = School::new();

        school.add_student(student_name, std_grade);

        let status: StudentStatus = StudentStatus::Active;

        school.update_student_status(1, status);

        let student = school.get_student(1);

        let s = student.unwrap().student_status.check_variants();
        assert_eq!(s, StudentStatus::Active);
    }

    #[test]
    fn update_student_grade() {
        let student_name = String::from("name");
        let std_grade: u8 = 40;

        let mut school: School = School::new();

        school.add_student(student_name, std_grade);

        school.update_student_grade(1, 70);

        let student = school.get_student(1).unwrap();

        assert_eq!(student.grade, 70);
    }

    #[test]
    fn create_multiple_students() {
        let student_name = String::from("name");
        let std_grade: u8 = 40;

        let mut school: School = School::new();

        school.add_student(student_name, std_grade);
        school.add_student("another".to_string(), 80);
        school.add_student("other".to_string(), 65);

        assert_eq!(school.student_key, 3);

        let student_2 = school.get_student(2);
        println!("{} {}", &student_2.unwrap().grade, 80);

        assert_eq!(&student_2.unwrap().grade, &80);
    }
}
