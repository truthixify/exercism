fn decode_plant(letter: char) -> &'static str {
    let plant = match letter {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => todo!(), 
    };

    plant
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_id: usize = (student.as_bytes()[0] - 65).into();
    let split_diagram_by_line: Vec<&str> = diagram.split("\n").collect();
    let first_row: Vec<char> = split_diagram_by_line[0].chars().collect();
    let second_row: Vec<char> = split_diagram_by_line[1].chars().collect();
    let mut result: Vec<&str> = vec![];

    result.push(decode_plant(first_row[2 * student_id]));
    result.push(decode_plant(first_row[2 * student_id + 1]));
    result.push(decode_plant(second_row[2 * student_id]));
    result.push(decode_plant(second_row[2 * student_id + 1]));

    result
}
