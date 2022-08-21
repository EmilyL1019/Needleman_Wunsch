use std::string;

// Transforms the vector of characters into one sequence string
fn char_to_string(characters: Vec<char>) -> String {
    let sequence: String = characters.into_iter().collect();
    return sequence;
}

// Find best alignment
fn priv_best_alignment<'a>(score_grid: &'a Vec<i32>,directions: &'a Vec<String>, cell: i32, seq1: &'a mut String, seq2: &'a mut String, aligned_seq1: &'a mut Vec<Vec<char>>, aligned_seq2: &'a mut Vec<Vec<char>>, aligned_seq_index: &'a mut usize) -> (Vec<String>, Vec<String>){
    let seq2_len = seq2.len() as i32 - 1;
    let seq1_char_index = ((cell - (cell % seq2_len as i32)) / seq2_len as i32) as usize;
    let seq2_char_index = (cell % seq2_len as i32) as usize;
    let seq1_char:char = seq1.chars().nth(seq1_char_index).unwrap();
    let seq2_char:char = seq2.chars().nth(seq2_char_index).unwrap();
    let direction_index = cell as usize;
    // Recursive cases: Go through directions;
    let mut new_aligned_seq1:&'a mut Vec<Vec<char>> = aligned_seq1;
    let mut new_aligned_seq2: &'a mut Vec<Vec<char>> = aligned_seq2;
    // Keeps track of how many directions the cell has
    let mut hasDiagonal:bool = false;
    let mut hasLeft:bool = false;
    // If the current direction index is D, add the two corresponding characters to the sequence strings    
    if directions[direction_index].contains("D") {
        let copy_aligned_seq1:&mut Vec<Vec<char>> = &mut new_aligned_seq1;
        let copy_aligned_seq2:&mut Vec<Vec<char>> = &mut new_aligned_seq2;
        //println!("Diagonal!");
        hasDiagonal = true;
        copy_aligned_seq1[*aligned_seq_index].insert(0, seq1_char);
        copy_aligned_seq2[*aligned_seq_index].insert(0, seq2_char);
        // Move to the cell to the diagonally left of the current cell
        if (direction_index as i32 - seq2_len - 1) as i32 >= 0 {
            priv_best_alignment(score_grid, directions, cell - seq2_len as i32 - 1, seq1, seq2, copy_aligned_seq1, copy_aligned_seq2, aligned_seq_index);
        }
    }
    // If the current direction index is L, add a gap to the sequence 1 string and the corresponding character to the sequence 2 string
    if directions[direction_index].contains("L") {
        let copy_aligned_seq1:&mut Vec<Vec<char>> = &mut new_aligned_seq1;
        let copy_aligned_seq2:&mut Vec<Vec<char>> = &mut new_aligned_seq2;
        hasLeft = true;
        // Check if cell has multiple paths
        if hasDiagonal {
           let new_index = *aligned_seq_index + 1;
            println!("{}", new_index);
            while new_index > *aligned_seq_index {    
                copy_aligned_seq1.insert(*aligned_seq_index + 1, vec![]);
                copy_aligned_seq2.insert(*aligned_seq_index + 1,vec![]);
                *aligned_seq_index = *aligned_seq_index + 1;
                // Copy old vector into new one
                //??

            }
        }
        copy_aligned_seq1[*aligned_seq_index].insert(0, '_');
        copy_aligned_seq2[*aligned_seq_index].insert(0, seq2_char);
        // Move to the cell to the left of the current cell
        if (direction_index as i32 - 1) as i32 >= 0 {
            priv_best_alignment(score_grid, directions, cell - 1 as i32, seq1, seq2, copy_aligned_seq1, copy_aligned_seq2, aligned_seq_index);
        }
    }
    // If the current direction index is U, add the corresponding character to the sequence 1 string and a gap to the sequence 2 string
    if directions[direction_index].contains("U") {
        let copy_aligned_seq1_2:&mut Vec<Vec<char>> = &mut new_aligned_seq1;
        let copy_aligned_seq2_2:&mut Vec<Vec<char>> = &mut new_aligned_seq2;
        print!("up!\n");
        // Check if cell has multiple paths
        if hasDiagonal || hasLeft {
            let new_index = *aligned_seq_index + 1;
            println!("{}", new_index);
            while new_index > *aligned_seq_index {    
                copy_aligned_seq1_2.insert(*aligned_seq_index + 1, vec![]);
                copy_aligned_seq2_2.insert(*aligned_seq_index + 1,vec![]);
                *aligned_seq_index = *aligned_seq_index + 1;
                // Copy old vector into new one
                //??
                
            }
        }
        copy_aligned_seq1_2[*aligned_seq_index].insert(0, seq1_char);
        copy_aligned_seq2_2[*aligned_seq_index].insert(0, '_');    
        // Move to the cell above the current cell
        if (direction_index as i32 - seq2_len) as i32 >= 0 {
            priv_best_alignment(score_grid, directions, cell - seq2_len as i32, seq1, seq2, copy_aligned_seq1_2, copy_aligned_seq2_2, aligned_seq_index);
        }
    }
    // Base case: Turn finished aligned sequences into strings and return them
    let mut str_aligned_seq1: Vec<String> = Vec::new();
    let mut str_aligned_seq2: Vec<String> = Vec::new();
    for i in 0..(new_aligned_seq1.len() as i32 - 1) {
        let char_aligned_seq1:Vec<char> = new_aligned_seq1[i as usize].to_vec().into_iter().collect();
        let char_aligned_seq2:Vec<char> = new_aligned_seq2[i as usize].to_vec().into_iter().collect();
        str_aligned_seq1.append(&mut vec![char_to_string(char_aligned_seq1)]);
        str_aligned_seq2.append(&mut vec![char_to_string(char_aligned_seq2)]);
    }
    return (str_aligned_seq1, str_aligned_seq2);
}
    
pub fn build_best_alignment<'a>(score_grid: &'a Vec<i32>,directions: &'a Vec<String>, cell: i32, seq1: &'a mut String, seq2: &'a mut String) -> (Vec<String>, Vec<String>){
    let mut aligned_sequence1:Vec<Vec<char>> = vec![[].to_vec()];
    let mut aligned_sequence2:Vec<Vec<char>> = vec![[].to_vec()];
    let mut str_aligned_seq1: Vec<String> = Vec::new();
    let mut str_aligned_seq2: Vec<String> = Vec::new();
    let mut index:usize = 0;
    (str_aligned_seq1, str_aligned_seq2) = priv_best_alignment(score_grid, directions, cell, seq1, seq2, &mut aligned_sequence1, &mut aligned_sequence2, &mut index);
    for i in 0..(aligned_sequence1.len() - 1) {
        let mut j = str_aligned_seq1[i].len();
        while j < str_aligned_seq1[i].len() {
            let char1 = str_aligned_seq1[0].chars().nth(j).unwrap();
            let char2 = str_aligned_seq2[0].chars().nth(j).unwrap();
            str_aligned_seq1[i].insert(j, char1);
            str_aligned_seq2[i].insert(j, char2);
            j = j + 1;
        }
    }
    return (str_aligned_seq1, str_aligned_seq2);
}

pub fn print_alignments(str_aligned_seq1: Vec<String>, str_aligned_seq2: Vec<String>) {
    for i in 0..str_aligned_seq1.len() {
        println!("{} {}", str_aligned_seq1[i], str_aligned_seq2[i]);
    }
}
