use csv::{ReaderBuilder, Result, WriterBuilder};
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Student {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Email")]
    email: String,
}
fn main() -> Result<()> {
    let file = File::open("students.csv")?;
    let mut reader_builder = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut students: Vec<Student> = reader_builder.deserialize().collect::<Result<_>>()?;

    let grouped_csv = File::create("grouped_student.csv")?;
    let mut writer_builder = WriterBuilder::new()
        .has_headers(true)
        .from_writer(grouped_csv);

    writer_builder.write_record(["Group", "Name", "Email"])?;

    let mut rand = thread_rng();
    students.shuffle(&mut rand);

    for (i, chunk) in students.chunks(3).enumerate() {
        let group_name = format!("Group {}", i + 1);
        for student in chunk {
            writer_builder.write_record([&group_name, &student.name, &student.email])?;
        }
    }

    writer_builder.flush()?;
    Ok(())
}
