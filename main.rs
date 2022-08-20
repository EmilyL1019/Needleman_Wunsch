use std::io;
mod grid;
mod alignment;
// ToDo:

fn main() {
    // Get two sequences as input
    let mut seq1 : String = String::new();
    io::stdin().read_line(&mut seq1).expect("Cannot read sequence 1.");
    let mut seq2 : String = String::new();
    io::stdin().read_line(&mut seq2).expect("Cannot read sequence 2.");
    let len1 = seq1.len() as i32 - 1;
    let len2 = seq2.len() as i32 - 1;
    let (grid, directions) = grid::create_grid(&mut seq1, &mut seq2, len1, len2);
    let mut counter = 0;
    for i in &grid {
        print!("{i} ");
        counter = counter + 1;
        if counter % (len2 + 1) == 0 {
            print!("\n");
        }    
    }
    let starting_cell = len2 + 2;
    let totalCells = (len1 + 1) * (len2 + 1) - 1;
    let (aligned_seq1, aligned_seq2) = alignment::build_best_alignment(&grid, &directions, totalCells, &mut seq1, &mut seq2);
    alignment::print_alignments(aligned_seq1, aligned_seq2);
}
#[cfg(test)]
mod tests {
    use crate::grid::create_grid;
    use crate::alignment::build_best_alignment;
    use crate::alignment::print_alignments;
    #[test]
    fn test1() {
        let grid: Vec<i32> = vec![0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10,
            -1, -1, -2, -3, -4, -5, -4, -5, -6, -7, -8,
            -2, -2, 0, -1, -2, -3, -4, -5, -6, -7, -8,
            -3, -3, -1, 1, 0, -1, -2, -3, -4, -5, -6,
            -4, -2, -2, 0, 2, 1, 0, -1, -2, -3, -4,
            -5, -3, -3, -1, 1, 1, 2, 1, 0, -1, -2,
            -6, -4, -4, -2, 0, 0, 2, 3, 2,  1, 0,
            -7, -5, -5, -3, -1, 1, 1, 2, 2, 1, 2,
            -8, -6, -4, -4, -2, 0, 0, 1, 1, 1, 1,
            -9, -7, -5, -3, -3, -1, -1, 0, 2, 2, 1,
            -10, -8, -6, -4, -4, -2, -2, -1, 1, 1, 1];
        let directions:Vec<String> = vec!["D".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(), "D".to_string(), "LD".to_string(), "L".to_string(), "L".to_string(), "L".to_string(),
        "UD".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(),
        "UD".to_string(), "U".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(), "LD".to_string(), "L".to_string(),
        "D".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(),
        "U".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "D".to_string(), "LD".to_string(), "L".to_string(), "L".to_string(), "L".to_string(),
        "U".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "L".to_string(),
        "UD".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(), "D".to_string(),
        "U".to_string(), "D".to_string(), "U".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "U".to_string(),
        "U".to_string(), "U".to_string(), "D".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "D".to_string(), "D".to_string(), "L".to_string(),
        "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "D".to_string()];
        let mut seq1 : String = "GTCAGGATCT\n".to_string();
        let mut seq2 : String = "ATCAAGGCCA\n".to_string();
        let (ftn_grid, ftn_directions) = create_grid(&mut seq1, &mut seq2, 10, 10);
        // Check values
        for i in 0..120 {
            assert_eq!(grid[i], ftn_grid[i]);
        }
        // Check directions
        for i in 0..100 {
            assert_eq!(directions[i], ftn_directions[i]);
        }
        let (aligned_seq1, aligned_seq2) = build_best_alignment(&ftn_grid, &ftn_directions, 99, &mut seq1, &mut seq2);
        print_alignments(aligned_seq1, aligned_seq2);
    }
    #[test]
    fn test2() {
        let grid = vec![0, -1, -2, -3, -4, -5,
        -1, -1, -2, -3, -2, -3,
        -2, -2, 0, -1, -2, -3,
        -3, -3, -1, 1, 0, -1,
        -4, -2, -2, 0, 0, -1,
        -5, -3, -3, -1, 1, 1,
        -6, -4, -4, -2, 0, 0,
        -7, -5, -5, -3, -1, -1,
        -8, -6, -6, -4, -2, 0];
        let directions:Vec<String> = vec!["D".to_string(), "LD".to_string(), "LD".to_string(), "D".to_string(), "LD".to_string(),
        "UD".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(),
        "UD".to_string(), "U".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), 
        "D".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(),
        "U".to_string(), "UD".to_string(), "U".to_string(), "D".to_string(), "D".to_string(), 
        "U".to_string(), "UD".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(),
        "U".to_string(), "UD".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), 
        "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "D".to_string()];
        let mut seq1 : String = "ATGCAGGA\n".to_string();
        let mut seq2 : String = "CTGAA\n".to_string();
        let (ftn_grid, ftn_directions) = create_grid(&mut seq1, &mut seq2, 8, 5);
        // Check values
        for i in 0..54 {
            assert_eq!(grid[i], ftn_grid[i]);
        }
        // Check directions
        for i in 0..39 {
            assert_eq!(directions[i], ftn_directions[i]);
        }
        let (aligned_seq1, aligned_seq2) = build_best_alignment(&ftn_grid, &ftn_directions, 53, &mut seq1, &mut seq2);
        assert_eq!(aligned_seq1, vec!["ATGCAGGA"]);
        assert_eq!(aligned_seq2, vec!["CTG-A--A"])
    }
   #[test]
    fn test3() {
        let grid = vec![0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15,
        -1, -1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13,
        -2, -2, 0, -1, -2, -3, -2, -3, -4,  -5, -6, -7, -8, -9, -10, -11,
        -3, -3, -1, -1, -2, -3, -3, -1, -2, -3, -4, -5, -6, -7, -8, -9,
        -4, -4, -2, 0, 0, -1, -2, -2, -2, -3, -4, -5, -6, -5, -6, -7,
        -5, -5, -3, -1, -1, -1, 0, -1, -2, -1, -2, -3, -4, -5, -6, -7,
        -6, -6, -4, -2, -2, -2, 0, -1, -2, -1, 0, -1, -2, -3, -4, -5,
        -7, -7, -5, -3, -3, -3, -1, 1, 0, -1, -1, 1, 0, -1, -2, -3,
        -8, -8, -6, -4, -4, -4, -2, 0, 2, 1, 0, 0, 0, -1, 0, -1,
        -9, -9, -7, -5, -3, -4, -3, -1, 1, 1, 0, -1, -1, 1, 0, 1,
        -10, -10, -8, -6, -4, -4, -4, -2, 0, 0, 0, 1, 0, 0, 2, 1,
        -11, -9, -9, -7, -5, -3, -4, -3, -1, -1, -1, 0, 2, 1, 1, 1,
        -12, -10, -8, -8, -6, -4, -2, -3, -2, 0, 0, -1, 1, 1, 0, 0,
        -13, -11, -9, -9, -7, -5, -3, -1, -2, -1, -1, 1, 0, 0, 2, 1,
        -14, -12, -10, -10, -8, -6, -4, -2, -2, -1, 0, 0, 0, -1, 1, 1,
        -15, -13, -11, -11, -9, -7, -5, -3, -3, -1, 0, -1, -1, -1, 0, 0,
        -16, -14, -12, -10, -10, -8, -6, -4, -4, -2, -1, -1, -2, 0, -1, 1,
        -17, -15, -13, -11, -11, -9, -7, -5, -3, -3, -2, 0, -1, -1, 1, 0,
        -18, -16, -14, -12, -12, -10, -8, -6, -4, -2, -2, -1, -1, -2, 0, 0,
        -19, -17, -15, -13, -13, -11, -9, -7, -5, -3, -1, -2, -2, -2, -1, -1,
        -20, -18, -16, -14, -14, -12, -10, -8, -6, -4, -2, -2, -3, -3, -2, -3
        ];
        let directions:Vec<String> = vec!["D".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(), "LD".to_string(), "L".to_string(), "L".to_string(),  "L".to_string(), "L".to_string(), "L".to_string(),
        "UD".to_string(), "D".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(), "LD".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), 
        "UD".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(), "LD".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(),  "L".to_string(),
        "UD".to_string(), "U".to_string(), "D".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(), "D".to_string(), "L".to_string(), "LD".to_string(),
        "UD".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "D".to_string(), "LD".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(), "LD".to_string(), 
        "UD".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "UD".to_string(), "D".to_string(), "LD".to_string(), "LD".to_string(), "D".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), 
        "UD".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "UD".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(), "L".to_string(), "U".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "LD".to_string(), "L".to_string(), 
        "UD".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "UD".to_string(), "D".to_string(), "LD".to_string(), "D".to_string(), "L".to_string(),
        "UD".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "L".to_string(), "U".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(), "UDL".to_string(), "UD".to_string(), "D".to_string(), "L".to_string(), "D".to_string(),
        "UD".to_string(), "U".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "U".to_string(), "UD".to_string(), "UD".to_string(), "UD".to_string(), "D".to_string(), "D".to_string(), "L".to_string(), "U".to_string(), "D".to_string(), "L".to_string(),
        "D".to_string(), "U".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "L".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "UD".to_string(), "U".to_string(), "D".to_string(), "L".to_string(), "U".to_string(), "D".to_string(),
        "U".to_string(), "D".to_string(), "U".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "L".to_string(), "U".to_string(), "D".to_string(), "D".to_string(), "UL".to_string(), "U".to_string(), "D".to_string(), "UDL".to_string(), "UD".to_string(),
        "U".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "UL".to_string(), "UD".to_string(), "D".to_string(), "L".to_string(),
        "U".to_string(), "UD".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(),"U".to_string(), "D".to_string(), "D".to_string(), "D".to_string(), "U".to_string(), "D".to_string(), "UDL".to_string(), "U".to_string(), "D".to_string(),
        "U".to_string(), "UD".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "D".to_string(), "UDL".to_string(), "UD".to_string(), "D".to_string(), "U".to_string(), "UD".to_string(),
        "U".to_string(), "U".to_string(), "D".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "UDL".to_string(), "D".to_string(), "UL".to_string(), "D".to_string(),
        "U".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "L".to_string(), "U".to_string(), "D".to_string(), "UL".to_string(),
        "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "D".to_string(), "D".to_string(), "U".to_string(), "D".to_string(), "UDL".to_string(), "U".to_string(), "D".to_string(),
        "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "UL".to_string(), "UD".to_string(), "D".to_string(), "U".to_string(), "UD".to_string(),
        "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "U".to_string(), "UD".to_string(), "UD".to_string(), "D".to_string(), "UDL".to_string(), "UD".to_string(), "U".to_string(), "UD".to_string()
        ];
        let mut seq1 : String = "AAGTAAGGTGCAGAATGAAA\n".to_string();
        let mut seq2 : String = "CATTCAGGAAGCTGT\n".to_string();
        let (ftn_grid, ftn_directions) = create_grid(&mut seq1, &mut seq2, 20, 15);
        //  Check values
        for i in 0..335 {
            assert_eq!(grid[i], ftn_grid[i]); 
        }
        // Check directions
        for i in 0..300 {
            assert_eq!(directions[i], ftn_directions[i]);
        }        
    }
}