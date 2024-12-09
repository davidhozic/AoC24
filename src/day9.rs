use std::fs::read_to_string;



pub fn part_one() {
    let mut blocks = parse_input();
    let mut i = 0;
    let mut blocks_len: usize = blocks.len();

    // Defragment by copying taken blocks from end and putting them on the first block.
    while i < blocks_len {
        let c = blocks[i];
        if c == -1 {
            let mut j = blocks_len - 1;
            while blocks[j] == -1 {
                j -= 1;
            }
            blocks[i] = blocks[j];
            blocks[j] = -1;
            blocks_len = j;  // Virtually shrink size of the vector without modifying it.
        }
        i += 1;
    }

    // Sum of (file_id * block_index)
    let checksum: usize = blocks.iter().take_while(|x| **x != -1)
        .enumerate().map(|(i, fid)| i * *fid as usize).sum();

    println!("{checksum}");

}


pub fn part_two() {

}


/// Reads a disk map. The disk map contains [n_block_file, n_free_space, n_block_file, n_free_space, ...] sequence.
/// The disk map is transformed into Vec of [n x file_id, m x free space, ...] where free space is marked with -1.
pub fn parse_input() -> Vec<isize> {
    let input = read_to_string("./inputs/day9.txt").unwrap();
    let mut file_id = 0;

    input.chars().enumerate().map(
        |(i, n)| {
            let n = n.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                let fid = vec![file_id; n];
                file_id += 1;
                fid
            }
            else {
                vec![-1isize; n]
            }
        }
    ).flatten().collect()
}

