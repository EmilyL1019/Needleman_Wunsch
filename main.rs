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
    let (aligned_seq1, aligned_seq2) = alignment::build_best_alignment(&grid, &directions, len1 * len2 - 1, &mut seq1, &mut seq2);
    let score = alignment::score(&aligned_seq1[0], &aligned_seq2[0]);
    alignment::print_alignments(&aligned_seq1, &aligned_seq2, score);
}

#[cfg(test)]
mod tests {
    use crate::alignment::score;
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
        let score = score(&aligned_seq1[0], &aligned_seq2[0]);
        print_alignments(&aligned_seq1, &aligned_seq2, score);
        assert_eq!(aligned_seq1, vec!["GTC-AGGATCT", "GTCA-GGATCT", "GTC-AGGATCT", "GTCA-GGATCT"]);
        assert_eq!(aligned_seq2, vec!["ATCAAGG-CCA", "ATCAAGG-CCA", "ATCAAGGC-CA", "ATCAAGGC-CA"]);
        assert_eq!(score, 1);
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
        let (aligned_seq1, aligned_seq2) = build_best_alignment(&ftn_grid, &ftn_directions, 39, &mut seq1, &mut seq2);
        let score = score(&aligned_seq1[0],&aligned_seq2[0]);
        print_alignments(&aligned_seq1, &aligned_seq2, score);
        assert_eq!(aligned_seq1, vec!["ATGCAGGA"]);
        assert_eq!(aligned_seq2, vec!["CTG-A--A"]);
        assert_eq!(score, 0);
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
        // Create and check alignments
        let (aligned_seq1, aligned_seq2) = build_best_alignment(&ftn_grid, &ftn_directions, 299, &mut seq1, &mut seq2);
        let score = score(&aligned_seq1[0], &aligned_seq2[0]);
        print_alignments(&aligned_seq1, &aligned_seq2, score);
        assert_eq!(aligned_seq1, vec!["AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA",
        "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA",
        "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA",
        "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA",
        "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA", "AAGTAAGGTGCAGAATGAAA"]);
        assert_eq!(aligned_seq2, vec!["CATTCA-G-GAAG-CTG--T", "CATTCAG--GAAG-CTG--T", "CATTCAGG--AAG-CTG--T", "CATTCAGG-A-AG-CTG--T", "CATTCAGGA--AG-CTG--T", 
        "CATTCA-G-GAAGC-TG--T", "CATTCAG--GAAGC-TG--T", "CATTCAGG--AAGC-TG--T", "CATTCAGG-A-AGC-TG--T", "CATTCAGGA--AGC-TG--T", 
        "CATTCA-G-GAAG-CTG-T-", "CATTCAG--GAAG-CTG-T-", "CATTCAGG--AAG-CTG-T-", "CATTCAGG-A-AG-CTG-T-", "CATTCAGGA--AG-CTG-T-", 
        "CATTCA-G-GAAGC-TG-T-", "CATTCAG--GAAGC-TG-T-", "CATTCAGG--AAGC-TG-T-", "CATTCAGG-A-AGC-TG-T-", "CATTCAGGA--AGC-TG-T-", 
        "CATTCA-G-GAAG-CTGT--", "CATTCAG--GAAG-CTGT--", "CATTCAGG--AAG-CTGT--", "CATTCAGG-A-AG-CTGT--", "CATTCAGGA--AG-CTGT--",
        "CATTCA-G-GAAGC-TGT--", "CATTCAG--GAAGC-TGT--", "CATTCAGG--AAGC-TGT--", "CATTCAGG-A-AGC-TGT--", "CATTCAGGA--AGC-TGT--"]);
        assert_eq!(score, -2);
    }

    #[test]
    fn test4() {
        let grid = vec![0, -1, -2, -3, -4, -5, -6, -7, -8, -9,
        -1, -1, -2, -3, -4, -3, -4, -5, -6, -7,
        -2, -2, -2, -1, -2, -3, -4, -5, -6, -7,
        -3, -1, -1, -2, -2, -3, -2, -3, -4, -5,
        -4, -2, -2, -2, -3, -3, -3, -1, -2, -3,
        -5, -3, -3, -3, -3, -2, -3, -2, -2, -3,
        -6, -4, -4, -2, -2, -3, -3, -3, -3, -3
        ];
        let directions:Vec<String> = vec!["D".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), "L".to_string(),
        "UD".to_string(), "D".to_string(), "D".to_string(), "LD".to_string(), "L".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(),
        "D".to_string(), "D".to_string(), "UL".to_string(), "D".to_string(), "LD".to_string(), "D".to_string(), "L".to_string(), "LD".to_string(), "LD".to_string(),
        "U".to_string(), "UD".to_string(), "D".to_string(), "UDL".to_string(), "D".to_string(), "U".to_string(), "D".to_string(), "L".to_string(), "L".to_string(),
        "U".to_string(), "UD".to_string(), "UD".to_string(), "D".to_string(), "D".to_string(), "L".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(),
        "U".to_string(), "UD".to_string(), "D".to_string(), "D".to_string(), "UL".to_string(), "D".to_string(), "U".to_string(), "UD".to_string(), "D".to_string()
        ];
        let mut seq1 : String = "TGACTG\n".to_string();
        let mut seq2 : String = "AAGGTACAA\n".to_string();
        let (ftn_grid, ftn_directions) = create_grid(&mut seq1, &mut seq2, 6, 9);
        //  Check values
        for i in 0..69 {
            assert_eq!(grid[i], ftn_grid[i]); 
        }
        // Check directions
        for i in 0..53 {
            assert_eq!(directions[i], ftn_directions[i]);
        }   
        // Create and check alignments
        let (aligned_seq1, aligned_seq2) = build_best_alignment(&ftn_grid, &ftn_directions, 53, &mut seq1, &mut seq2);
        let score = score(&aligned_seq1[0], &aligned_seq2[0]);
        print_alignments(&aligned_seq1, &aligned_seq2, score);
        assert_eq!(aligned_seq1, vec!["--TG-ACTG", "-T-G-ACTG", "T--G-ACTG", "-TG--ACTG", "T-G--ACTG"]);
        assert_eq!(aligned_seq2, vec!["AAGGTACAA", "AAGGTACAA", "AAGGTACAA", "AAGGTACAA", "AAGGTACAA"]);
        assert_eq!(score, -3);
    }

    #[test]
    fn test5() {
        let grid = vec![0, -1, -2, -3, -4, -5, -6,
        -1, -1, -2, -1, -2, -3, -4,
        -2, 0, 0, -1, -2, -3, -2,
        -3, -1, -1, -1, 0, -1, -2,
        -4, -2, -2, -2, -1, 1, 0,
        -5, -3, -3, -3, -1, 0, 0,
        -6, -4, -2, -3, -2, -1, 1,
        -7, -5, -3, -3, -3, -1, 0,
        -8, -6, -4, -4, -2, -2, -1,
        -9, -7, -5, -5, -3, -1, -2
        ];
        let directions:Vec<String> = vec!["D".to_string(), "LD".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "L".to_string(), 
        "D".to_string(), "D".to_string(), "L".to_string(), "LD".to_string(), "LD".to_string(), "D".to_string(), 
        "U".to_string(), "UD".to_string(), "D".to_string(), "D".to_string(), "L".to_string(), "L".to_string(),
        "U".to_string(), "UD".to_string(), "UD".to_string(), "U".to_string(), "D".to_string(), "L".to_string(), 
        "U".to_string(), "UD".to_string(), "UD".to_string(), "D".to_string(), "U".to_string(), "D".to_string(), 
        "UD".to_string(), "D".to_string(), "L".to_string(), "U".to_string(), "U".to_string(), "D".to_string(),
        "U".to_string(), "U".to_string(), "D".to_string(), "U".to_string(), "D".to_string(), "U".to_string(), 
        "U".to_string(), "U".to_string(), "UD".to_string(), "D".to_string(), "U".to_string(), "U".to_string(), 
        "U".to_string(), "U".to_string(), "UD".to_string(), "U".to_string(), "D".to_string(), "UL".to_string()
        ];
        let mut seq1 : String = "CTAGATGAG\n".to_string();
        let mut seq2 : String = "TTCAGT\n".to_string();
        let (ftn_grid, ftn_directions) = create_grid(&mut seq1, &mut seq2, 9, 6);
        //  Check values
        for i in 0..69 {
            assert_eq!(grid[i], ftn_grid[i]); 
        }
        // Check directions
        for i in 0..53 {
            assert_eq!(directions[i], ftn_directions[i]);
        }   
        // Create and check alignments
        let (aligned_seq1, aligned_seq2) = build_best_alignment(&ftn_grid, &ftn_directions, 53, &mut seq1, &mut seq2);
        let score = score(&aligned_seq1[0], &aligned_seq2[0]);
        print_alignments(&aligned_seq1, &aligned_seq2, score);
        assert_eq!(aligned_seq1, vec!["CTAGATGAG-", "CT-AGATGAG"]);
        assert_eq!(aligned_seq2, vec!["-T---TCAGT", "TTCAG-T---"]);
        assert_eq!(score, -2);
    }

    #[test]
    fn test6() {
        let grid = vec![0, -1, -2, -3, -4, -5, -6, -7, -8, 
        -1, -1, -2, -3, -4, -3, -4, -5, -6,
        -2, -2, -2, -3, -4, -3, -4, -5, -6,
        -3, -3, -3, -3, -4, -3, -4, -5, -6,
        -4, -4, -4, -4, -4, -4, -4, -5, -6,
        -5, -3, -3, -3, -4, -5, -3, -4, -4,
        -6, -4, -4, -4, -4, -3, -4, -4, -5,
        -7, -5, -5, -5, -5, -4, -4, -5, -5,
        -8, -6, -6, -6, -6, -4, -5, -5, -6
        ];
        let directions:Vec<String> = vec!["D".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(), "D".to_string(), "L".to_string(), "L".to_string(), "L".to_string(),
        "UD".to_string(), "D".to_string(), "LD".to_string(), "LD".to_string(), "D".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(),
        "UD".to_string(), "UD".to_string(), "D".to_string(), "LD".to_string(), "D".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(),
        "UD".to_string(), "UD".to_string(), "UD".to_string(), "D".to_string(), "U".to_string(), "D".to_string(), "LD".to_string(), "LD".to_string(),
        "D".to_string(), "D".to_string(), "D".to_string(), "L".to_string(), "UDL".to_string(), "D".to_string(), "L".to_string(), "D".to_string(),
        "U".to_string(), "UD".to_string(), "UD".to_string(), "D".to_string(), "D".to_string(), "UL".to_string(), "D".to_string(), "UDL".to_string(), 
        "U".to_string(), "UD".to_string(), "UD".to_string(), "UD".to_string(), "U".to_string(), "D".to_string(), "UDL".to_string(), "D".to_string(), 
        "U".to_string(), "UD".to_string(), "UD".to_string(), "UD".to_string(), "D".to_string(), "UDL".to_string(), "D".to_string(), "UDL".to_string()
        ];
        let mut seq1 : String = "TTTGATGT\n".to_string();
        let mut seq2 : String = "AAACTACA\n".to_string();
        let (ftn_grid, ftn_directions) = create_grid(&mut seq1, &mut seq2, 8, 8);
        //  Check values
        for i in 0..80 {
            assert_eq!(grid[i], ftn_grid[i]); 
        }
        // Check directions
        for i in 0..63 {
            assert_eq!(directions[i], ftn_directions[i]);
        }   
        // Create and check alignments
        let (aligned_seq1, aligned_seq2) = build_best_alignment(&ftn_grid, &ftn_directions, 63, &mut seq1, &mut seq2);
        let score = score(&aligned_seq1[0], &aligned_seq2[0]);
        print_alignments(&aligned_seq1, &aligned_seq2, score);
        assert_eq!(aligned_seq1, vec!["TTTGA-T-GT", "TTTGA-T-GT", "TTTGA-T-GT", "TTTGA-T-GT", "TTTGA-T-GT", 
        "TTTGA-T-GT", "--TTTGATGT", "-T-TTGATGT", "T--TTGATGT", "-TT-TGATGT", 
        "T-T-TGATGT", "TT--TGATGT", "TTTGA-TG-T", "TTTGA-TG-T", "TTTGA-TG-T", 
        "TTTGA-TG-T", "TTTGA-TG-T", "TTTGA-TG-T", "--TTTGATGT", "-T-TTGATGT", 
        "T--TTGATGT", "-TT-TGATGT", "T-T-TGATGT", "TT--TGATGT", "TTTGA-TGT-", 
        "TTTGA-TGT-", "TTTGA-TGT-", "TTTGA-TGT-", "TTTGA-TGT-", "TTTGA-TGT-", 
        "--TTTGATGT", "-T-TTGATGT", "T--TTGATGT", "-TT-TGATGT", "T-T-TGATGT",
        "TT--TGATGT"]);
        assert_eq!(aligned_seq2, vec!["--AAACTACA", "-A-AACTACA", "A--AACTACA", "-AA-ACTACA", "A-A-ACTACA", 
        "AA--ACTACA", "AAACT-A-CA", "AAACT-A-CA", "AAACT-A-CA", "AAACT-A-CA", 
        "AAACT-A-CA", "AAACT-A-CA", "--AAACTACA", "-A-AACTACA", "A--AACTACA", 
        "-AA-ACTACA", "A-A-ACTACA", "AA--ACTACA", "AAACT-AC-A", "AAACT-AC-A",
        "AAACT-AC-A", "AAACT-AC-A", "AAACT-AC-A", "AAACT-AC-A", "--AAACTACA",
        "-A-AACTACA", "A--AACTACA", "-AA-ACTACA", "A-A-ACTACA", "AA--ACTACA", 
        "AAACT-ACA-", "AAACT-ACA-", "AAACT-ACA-", "AAACT-ACA-", "AAACT-ACA-",
        "AAACT-ACA-"]);
        assert_eq!(score, -6);
    }

    #[test]
    fn test7() {
        let grid = vec![0, -1, -2, -3, -4, -5,
        -1, -1, -2, -3, -4, -5,
        -2, -2, -2, -3, -4, -5,
        -3, -3, -3, -3, -4, -5,
        -4, -4, -4, -4, -4, -5,
        -5, -5, -5, -5, -5, -5
        ];
        let directions:Vec<String> = vec!["D".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(),
        "UD".to_string(), "D".to_string(), "LD".to_string(), "LD".to_string(), "LD".to_string(),
        "UD".to_string(), "UD".to_string(), "D".to_string(), "LD".to_string(), "LD".to_string(),
        "UD".to_string(), "UD".to_string(), "UD".to_string(), "D".to_string(), "LD".to_string(),
        "UD".to_string(), "UD".to_string(), "UD".to_string(), "UD".to_string(), "D".to_string(),
        ];
        let mut seq1 : String = "AAAAA\n".to_string();
        let mut seq2 : String = "TTTTT\n".to_string();
        let (ftn_grid, ftn_directions) = create_grid(&mut seq1, &mut seq2, 5, 5);
        //  Check values
        for i in 0..35 {
            assert_eq!(grid[i], ftn_grid[i]); 
        }
        // Check directions
        for i in 0..24 {
            assert_eq!(directions[i], ftn_directions[i]);
        }   
        // Create and check alignments
        let (aligned_seq1, aligned_seq2) = build_best_alignment(&ftn_grid, &ftn_directions, 24, &mut seq1, &mut seq2);
        let score = score(&aligned_seq1[0], &aligned_seq2[0]);
        print_alignments(&aligned_seq1, &aligned_seq2, score);
        assert_eq!(aligned_seq1, vec!["AAAAA"]);
        assert_eq!(aligned_seq2, vec!["TTTTT"]);
        assert_eq!(score, -5);
    }

}
