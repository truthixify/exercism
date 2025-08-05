use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.len() == second_list.len() {
        if first_list == second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else if first_list.len() > second_list.len() {
        if second_list.is_empty() {
            Comparison::Superlist
        } else {
            let second_list_first_el = second_list[0];
            let second_list_last_el = second_list[second_list.len() - 1];
            if let (Some(first_pos), Some(last_pos)) = (first_list.iter().position(|&x| x == second_list_first_el), first_list.iter().position(|&x| x == second_list_last_el)) {
                match (last_pos + 1 - first_pos).cmp(&second_list.len()) {
                    Ordering::Greater => {
                        let mut i = first_pos + 1;
                        let mut j = last_pos - 1;

                        while i != last_pos {
                            if &first_list[i..=last_pos] == second_list {
                                return Comparison::Superlist
                            }

                            i += 1;
                        }

                        while j != first_pos {
                            if &first_list[first_pos..=j] == second_list {
                                return Comparison::Superlist
                            }

                            j -= 1;
                        }

                        while i != last_pos && j != first_pos {
                            if &first_list[i..=j] == second_list {
                                return Comparison::Superlist
                            }

                            i += 1;
                            j -= 1;
                        }

                        Comparison::Unequal
                    },
                    Ordering::Equal => {
                        if &first_list[first_pos..=last_pos] == second_list {
                            Comparison::Superlist
                        } else {
                            Comparison::Unequal
                        }
                    }
                    Ordering::Less => Comparison::Unequal,
                }
            } else {
                Comparison::Unequal
            }
        }
    } else if first_list.is_empty() {
        Comparison::Sublist
    } else {
        let first_list_first_el = first_list[0];
        let first_list_last_el = first_list[first_list.len() - 1];
        if let (Some(first_pos), Some(last_pos)) = (second_list.iter().position(|&x| x == first_list_first_el), second_list.iter().position(|&x| x == first_list_last_el)) {
            match (last_pos + 1 - first_pos).cmp(&first_list.len()) {
                Ordering::Greater => {
                    let mut i = first_pos + 1;
                    let mut j = last_pos - 1;

                    while i != last_pos {
                        if &second_list[i..=last_pos] == first_list {
                            return Comparison::Sublist;
                        }

                        i += 1;
                    }

                    while j != first_pos {
                        if &second_list[first_pos..=j] == first_list {
                            return Comparison::Sublist;
                        }

                        j -= 1;
                    }

                    while i != last_pos && j != first_pos {
                        if &second_list[i..=j] == first_list {
                            return Comparison::Sublist;
                        }

                        i += 1;
                        j -= 1;
                    }
                    
                    Comparison::Unequal
                },
                Ordering::Equal => {
                    if &second_list[first_pos..=last_pos] == first_list {
                        Comparison::Sublist
                    } else {
                        Comparison::Unequal
                    }
                },
                Ordering::Less => Comparison::Unequal,
            }
        } else {
            Comparison::Unequal
        }
    }
}
