use std::collections::VecDeque;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (files, empties) = parse_input(input);
    let ps = cumsum(component_sum(&files, &empties));
    let mut files = VecDeque::from(files);
    let checksum: usize = empties
        .iter()
        .enumerate()
        .map_while(|(i, e)| {
            if let Some(file_size) = files.pop_front() {
                let file_start = *ps.get(i).unwrap();
                let eval: usize = (0..*e)
                    .map_while(|j| {
                        let file_id = i + get_from_end(&mut files)?;
                        Some((file_start + file_size + j) * file_id)
                    })
                    .sum();
                Some(file_checksum(i, file_size, file_start) + eval)
            } else {
                None
            }
        })
        .sum();
    Ok(checksum.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (files, empties) = parse_input(input);
    let file_pos = cumsum(component_sum(&files, &empties));
    let mut empties: Vec<_> = cumsum(component_sum(&files[1..], &empties))
        .into_iter()
        .map(|p| p + files[0])
        .zip(empties)
        .collect(); // [(pos, size)]

    let files = file_pos.into_iter().zip(files); // [(pos, size)]

    let mut total = 0;
    for (f_id, (f_pos, f_size)) in files.enumerate().rev() {
        let pos = if let Some((ref mut e_pos, ref mut e_size)) = empties
            .iter_mut()
            .take_while(|(p, _)| *p < f_pos)
            .find(|(_, s)| *s >= f_size)
        {
            let new_pos = *e_pos;
            *e_pos += f_size;
            *e_size -= f_size;
            new_pos
        } else {
            f_pos
        };
        total += file_checksum(f_id, f_size, pos);
    }

    Ok(total.to_string())
}

fn parse_input(input: String) -> (Vec<usize>, Vec<usize>) {
    let input_int: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let files = input_int.iter().step_by(2).copied().collect();
    let empties = input_int.iter().skip(1).step_by(2).copied().collect();

    (files, empties)
}

// TODO: make generic and move to shared library
// Consider adding trait for iterators, or using external crate
fn cumsum(xs: Vec<usize>) -> Vec<usize> {
    std::iter::once(0)
        .chain(xs.iter().scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        }))
        .collect()
}

fn component_sum(xs: &[usize], ys: &[usize]) -> Vec<usize> {
    xs.iter().zip(ys.iter()).map(|(x, y)| x + y).collect()
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

fn file_checksum(id: usize, size: usize, pos: usize) -> usize {
    id * size * (2 * pos + size - 1) / 2
}
