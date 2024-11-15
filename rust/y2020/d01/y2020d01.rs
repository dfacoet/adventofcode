use core::panic;

pub fn part1(input: String) -> String {
    let ns: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
    for i in 0..(ns.len() - 1) {
        for j in (i + 1)..(ns.len() - 1) {
            if ns[i] + ns[j] == 2020 {
                return (ns[i] * ns[j]).to_string();
            }
        }
    }
    panic!("!")
}

pub fn part2(input: String) -> String {
    let ns: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
    for i in 0..(ns.len() - 1) {
        for j in (i + 1)..(ns.len() - 1) {
            for k in (j + 1)..(ns.len() - 1) {
                if ns[i] + ns[j] + ns[k] == 2020 {
                    return (ns[i] * ns[j] * ns[k]).to_string();
                }
            }
        }
    }
    panic!("!")
}
