use std::collections::HashMap;

solution!(
    "Day 8: Seven Segment Search",
    || {
        common::load("day_08")
            .lines()
            .flat_map(|x| x
                .split(" | ")
                .collect::<Vec<&str>>()[1]
                .split(" ")
                .collect::<Vec<&str>>()
            )
            .map(|x| [2, 3, 4, 7].contains(&x.len()) as usize)
            .sum::<usize>()
            .to_string()
    },
    || {
        let data = common::load("day_08");
        let parsed = data
            .lines()
            .map(|x| x
                .split(" | ")
                .map(|x| x
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                )
                .collect::<Vec<Vec<&str>>>()
            )
            .collect::<Vec<Vec<Vec<&str>>>>();
        let mut sum = 0;
        for p in parsed.iter() {
            sum += compute(p.to_vec());
        }
        sum.to_string()
    }
);

fn compute(data: Vec<Vec<&str>>) -> usize {
    let entries = &data[0];
    let outputs = &data[1];
    let mut numbers = HashMap::<usize, &str>::new();

    while numbers.len() < 10 {
        for entry in entries {
            if entry.len() == 2 {
                numbers.insert(1, entry);
                continue;
            } else if entry.len() == 3 {
                numbers.insert(7, entry);
                continue;
            } else if entry.len() == 4 {
                numbers.insert(4, entry);
                continue;
            } else if entry.len() == 7 {
                numbers.insert(8, entry);
                continue;
            } else if entry.len() == 6 {
                if numbers.contains_key(&1) && str_contains(entry, char(numbers[&1], 0)) ^ str_contains(entry, char(numbers[&1], 1)) {
                    numbers.insert(6, entry);
                    continue;
                }
                if numbers.contains_key(&3) && diffs(&entry, numbers[&3]) == 1 {
                    numbers.insert(9, entry);
                    continue;
                }
                if numbers.contains_key(&6) && entry != &numbers[&6] && numbers.contains_key(&9) && entry != &numbers[&9] {
                    numbers.insert(0, entry);
                    continue;
                }
            } else if entry.len() == 5 {
                if numbers.contains_key(&1) && str_contains(entry, char(numbers[&1], 0)) && str_contains(entry, char(numbers[&1], 1)) {
                    numbers.insert(3, entry);
                    continue
                }
                if numbers.contains_key(&6) && diffs(&entry, numbers[&6]) == 1 {
                    numbers.insert(5, entry);
                    continue;
                }
                if numbers.contains_key(&3) && entry != &numbers[&3] && numbers.contains_key(&5) && entry != &numbers[&5] {
                    numbers.insert(2, entry);
                    continue;
                }
            }
        }
    }
    decode(outputs, numbers)
}

fn decode(encoded: &Vec<&str>, numbers: HashMap<usize, &str>) -> usize {
    let mut decipher = String::from("0");
    for cipher in encoded {
        // if let Some(n) = numbers.iter().find_map(|(k, v)| if v == cipher { Some(k) } else { None }) {
        //     println!("{:?}", n);
        //     decipher.push_str(&n.to_string());
        // }
        for (k, v) in numbers.iter() {
            if diffs(cipher, v) == 0 {
                decipher.push_str(&*k.to_string());
            }
        }
    }
    decipher.parse().unwrap()
}

fn char(word: &str, position: usize) -> char {
    word.chars().nth(position).unwrap()
}

fn str_contains(string: &str, c: char) -> bool {
    string.chars().collect::<Vec<char>>().contains(&c)
}

fn diffs(a: &str, b: &str) -> usize {
    let mut same: usize = 0;
    for c in a.chars() {
        same += str_contains(&b, c) as usize
    }
    std::cmp::max(a.len(), b.len()) - same
}

/*
 aaaa   1 [ ,  , c,  ,  , f,  ]
b    c  2 [a,  , c, d, e,  , g]
b    c  3 [a,  , c, d,  , f, g]
 dddd   4 [ , b, c, d,  , f,  ]
e    f  5 [a, b,  , d,  , f, g]
e    f  6 [a, b,  , d, e, f, g]
 gggg   7 [a,  , c,  ,  , f,  ]
        8 [a, b, c, d, e, f, g]
        9 [a, b, c, d,  , f, g]
        0 [a, b, c,  , e, f, g]

a [ ,  , c,  ,  , f,  ]
- [a,  , c,  ,  , f,  ]

find 1: length = 2
find 4: length = 4
find 7: length = 3
find 8: length = 7
find 9: length = 6 and has both 1's segments
find 6: length = 6 and is not 9
find 0: last one with length = 6
seg e: seg not on 9
seg c: seg not on 6
seg f: other seg of 1
seg a: other seg of 7
find 3: length = 5 and has both 1's segments
find 2: length = 5 and doesn't have segment f
find 5: length = 5 and doesn't have segment c
 */