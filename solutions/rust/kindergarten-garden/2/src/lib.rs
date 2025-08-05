fn decode_plant(letter: char) -> &'static str {
    match letter {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => todo!(), 
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_id: usize = (student.as_bytes()[0] - 65).into();
    let split_diagram_by_line: Vec<&str> = diagram.split("\n").collect();
    let first_row: Vec<char> = split_diagram_by_line[0].chars().collect();
    let second_row: Vec<char> = split_diagram_by_line[1].chars().collect();
    
    vec![first_row[2 * student_id], first_row[2 * student_id + 1], second_row[2 * student_id], second_row[2 * student_id + 1]].iter().map(|&c| decode_plant(c)).collect()
}
