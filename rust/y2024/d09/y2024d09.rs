use std::collections::VecDeque;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (mut files, empties) = parse_input(input);
    let ps: Vec<_> = std::iter::once(0)
        .chain(
            files
                .iter()
                .zip(empties.iter())
                .map(|(x, y)| x + y)
                .scan(0, |acc, x| {
                    *acc += x; // cumsum
                    Some(*acc)
                }),
        )
        .collect();
    let checksum: usize = empties
        .iter()
        .enumerate()
        .map_while(|(i, e)| {
            if let Some(file_n_blocks) = files.pop_front() {
                let file_start = *ps.get(i).unwrap();
                let eval: usize = (0..*e)
                    .map_while(|j| {
                        let file_id = i + get_from_end(&mut files)?;
                        Some((file_start + file_n_blocks + j) * file_id)
                    })
                    .sum();
                Some(file_checksum(i, file_n_blocks, file_start) + eval)
            } else {
                None
            }
        })
        .sum();
    Ok(checksum.to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

fn parse_input(input: String) -> (VecDeque<usize>, Vec<usize>) {
    let input_int: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let files = input_int.iter().step_by(2).copied().collect();
    let empties = input_int.iter().skip(1).step_by(2).copied().collect();

    (files, empties)
}

fn get_from_end(files: &mut VecDeque<usize>) -> Option<usize> {
    while files.back() == Some(&0) {
        files.pop_back();
    }
    if let Some(last) = files.back_mut() {
        *last -= 1;
        Some(files.len())
    } else {
        None
    }
}

fn file_checksum(id: usize, n_blocks: usize, pos: usize) -> usize {
    id * n_blocks * (2 * pos + n_blocks - 1) / 2
}
