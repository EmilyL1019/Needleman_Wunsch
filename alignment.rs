// Transforms the vector of characters into one sequence string
fn char_to_string(characters: Vec<char>) -> String {
    let sequence: String = characters.into_iter().collect();
    return sequence;
}

// Find best alignment
fn priv_best_alignment<'a>(score_grid: &'a Vec<i32>,directions: &'a Vec<String>, cell: i32, seq1: &'a mut String, seq2: &'a mut String, aligned_seq1: &'a mut Vec<Vec<char>>, aligned_seq2: &'a mut Vec<Vec<char>>, aligned_seq_index: &'a mut usize) -> (Vec<String>, Vec<String>){
    let seq2_len = seq2.len() as i32 - 1;
    let mut seq1_char_index = ((cell - (cell % seq2_len as i32)) / seq2_len as i32) as usize;
    let mut seq2_char_index = (cell % seq2_len as i32) as usize;
    let seq1_char:char = seq1.chars().nth(seq1_char_index).unwrap();
    let seq2_char:char = seq2.chars().nth(seq2_char_index).unwrap();
    let direction_index = cell as usize;
    // Recursive cases: Go through directions;
    let mut new_aligned_seq1:&'a mut Vec<Vec<char>> = aligned_seq1;
    let mut new_aligned_seq2: &'a mut Vec<Vec<char>> = aligned_seq2;
    // Keeps track of how many directions the cell has
    let mut has_diagonal:bool = false;
    let mut has_left:bool = false;
    // If the current direction index is D, add the two corresponding characters to the sequence strings    
    if directions[direction_index].contains("D") {
        let copy_aligned_seq1:&mut Vec<Vec<char>> = &mut new_aligned_seq1;
        let copy_aligned_seq2:&mut Vec<Vec<char>> = &mut new_aligned_seq2;
        has_diagonal = true;
        copy_aligned_seq1[*aligned_seq_index].insert(0, seq1_char);
        copy_aligned_seq2[*aligned_seq_index].insert(0, seq2_char);
        // Move to the cell to the diagonally left of the current cell
        if (direction_index as i32 - seq2_len - 1) as i32 >= 0 && seq1_char_index > 0 && seq2_char_index > 0{
            priv_best_alignment(score_grid, directions, cell - seq2_len as i32 - 1, seq1, seq2, copy_aligned_seq1, copy_aligned_seq2, aligned_seq_index);
        }
        // Add gaps if necessary to have two sequences of equal length
        else {
            while seq1_char_index > 0 as usize{
                seq1_char_index = seq1_char_index - 1;
                copy_aligned_seq1[*aligned_seq_index].insert(0, seq1.chars().nth(seq1_char_index).unwrap());
                copy_aligned_seq2[*aligned_seq_index].insert(0, '-');
            }
            while seq2_char_index > 0 as usize{
                seq2_char_index = seq2_char_index - 1;
                copy_aligned_seq1[*aligned_seq_index].insert(0, '-' );
                copy_aligned_seq2[*aligned_seq_index].insert(0, seq2.chars().nth(seq2_char_index).unwrap());
            }
        }
    }
    // If the current direction index is L, add a gap to the sequence 1 string and the corresponding character to the sequence 2 string
    if directions[direction_index].contains("L") {
        let copy_aligned_seq1:&mut Vec<Vec<char>> = &mut new_aligned_seq1;
        let copy_aligned_seq2:&mut Vec<Vec<char>> = &mut new_aligned_seq2;
        has_left = true;
        // Check if cell has multiple paths
        if has_diagonal {
           let new_index = *aligned_seq_index + 1;
            while new_index > *aligned_seq_index {    
                copy_aligned_seq1.insert(*aligned_seq_index + 1, vec![]);
                copy_aligned_seq2.insert(*aligned_seq_index + 1,vec![]);
                *aligned_seq_index = *aligned_seq_index + 1;
            }
        }
        copy_aligned_seq1[*aligned_seq_index].insert(0, '-');
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
        // Check if cell has multiple paths
        if has_diagonal || has_left {
            let new_index = *aligned_seq_index + 1;
            while new_index > *aligned_seq_index {    
                copy_aligned_seq1_2.insert(*aligned_seq_index + 1, vec![]);
                copy_aligned_seq2_2.insert(*aligned_seq_index + 1,vec![]);
                *aligned_seq_index = *aligned_seq_index + 1;
            }
        }
        copy_aligned_seq1_2[*aligned_seq_index].insert(0, seq1_char);
        copy_aligned_seq2_2[*aligned_seq_index].insert(0, '-');    
        // Move to the cell above the current cell
        if (direction_index as i32 - seq2_len) as i32 >= 0 {
            priv_best_alignment(score_grid, directions, cell - seq2_len as i32, seq1, seq2, copy_aligned_seq1_2, copy_aligned_seq2_2, aligned_seq_index);
        }
    }
    // Base case: Turn finished aligned sequences into strings and return them
    let mut str_aligned_seq1: Vec<String> = Vec::new();
    let mut str_aligned_seq2: Vec<String> = Vec::new();
    for i in 0..(new_aligned_seq1.len() as i32) {
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
    for i in 0..aligned_sequence1.len() {
        let mut j = str_aligned_seq1[i].len();
        while j < str_aligned_seq1[0].len() {
            let char1 = str_aligned_seq1[i - 1].chars().nth(j).unwrap();
            let char2 = str_aligned_seq2[i - 1].chars().nth(j).unwrap();
            str_aligned_seq1[i].insert(j, char1);
            str_aligned_seq2[i].insert(j, char2);
            j = j + 1;
        }
    }
    return (str_aligned_seq1, str_aligned_seq2);
}

// Calculates the score
pub fn score<'a>(str_aligned_seq1: &'a String, str_aligned_seq2: &'a String) -> i32{
    let mut score: i32 = 0;
    for i in 0..str_aligned_seq1.len(){
        // Check for match and add 1 to score
        if str_aligned_seq1.chars().nth(i) == str_aligned_seq2.chars().nth(i) {
            score = score + 1;
        }
        else {
            score = score - 1
        }
    }
    return score;
}

// Prints all alignments
pub fn print_alignments<'a>(str_aligned_seq1: &'a Vec<String>, str_aligned_seq2: &'a Vec<String>, score: i32) {
    println!("Best Alignments:");
    for i in 0..str_aligned_seq1.len() {
        println!("{}      {}", str_aligned_seq1[i], str_aligned_seq2[i]);
    }
    println!("Score: {}", score)
}
