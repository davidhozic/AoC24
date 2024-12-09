use std::fs::read_to_string;


pub fn part_one() {
    let mut blocks = parse_input_1();
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
    let mut blocks = parse_input_2();
    let block_fids: Vec<_> = blocks.iter().map(|(x, _)| *x).filter(|x| *x != -1).collect();

    for fid in block_fids.into_iter().rev() {
        let fid_pos = blocks.iter().position(|(bfid, _)| *bfid == fid).unwrap();
        let (_, flength) = blocks[fid_pos];
        let empty_poso = blocks.iter().position(|(bfid, length)| *bfid == -1 && *length >= flength);

        if let Some(empty_pos) = empty_poso {
            if empty_pos >= fid_pos {  // We have to move to the left
                continue;
            }

            // Can be moved => move it.
            let (_, empty_length) = blocks[empty_pos];
            let leftover = empty_length - flength;
            blocks[empty_pos] = (fid, flength);
            blocks[fid_pos] = (-1, flength);


            if fid_pos + 1 < blocks.len() {
                if let (-1, n) = blocks[fid_pos + 1] {
                    blocks[fid_pos] = (-1, flength + n);
                    // We can remove this safely, because the leftover insert inserts to index below fid_pos.
                    blocks.remove(fid_pos + 1);
                }
            }

            if leftover > 0 {
                blocks.insert(empty_pos + 1, (-1, leftover));
            }
        }
        else {
            continue;  // Can't be moved.
        }
    }


    let mut checksum: usize = 0;
    let mut index = 0;
    for (b, length) in &blocks {
        if *b == -1 {
            index += length;
            continue;
        }

        for _ in 0..*length {
            checksum += index * *b as usize;
            index += 1;
        }
    }

    println!("{checksum}");
    // println!("{blocks:?}");
}


/// Reads a disk map. The disk map contains [n_block_file, n_free_space, n_block_file, n_free_space, ...] sequence.
/// The disk map is transformed into Vec of [n x file_id, m x free space, ...] where free space is marked with -1.
pub fn parse_input_1() -> Vec<isize> {
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

/// Reads a disk map. The disk map contains [n_block_file, n_free_space, n_block_file, n_free_space, ...] sequence.
/// The disk map is transformed into Vec of [(file_id, len)] where free space is marked with -1.
pub fn parse_input_2() -> Vec<(isize, usize)> {
    let input = read_to_string("./inputs/day9.txt").unwrap();
    let mut file_id = 0;

    input.chars().enumerate().map(
        |(i, n)| {
            let n = n.to_digit(10).unwrap() as usize;

            if i % 2 == 0 {
                let fid = (file_id, n);
                file_id += 1;
                fid
            }
            else {
                (-1, n)
            }
        }
    ).filter(|(_, n)| *n > 0).collect()
}

