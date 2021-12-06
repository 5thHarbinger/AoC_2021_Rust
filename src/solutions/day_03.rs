solution!(
    "Day 3: Binary Diagnostic",
    || {
        let data = common::load("day_03")
            .lines()
            .map(|l| String::from(l))
            .collect::<Vec<String>>();
        let mut gamma = String::from("");
        let bits = get_most_common(&data);

        for counts in bits {
            if counts.1 > counts.0 {
                gamma.push('1');
            } else {
                gamma.push('0');
            }
        }

        let gamma = u16::from_str_radix(&gamma, 2).unwrap();
        let epsilon = ! gamma & 0b0000111111111111;

        (gamma as usize * epsilon as usize).to_string()
    },
    || {
        let data = common::load("day_03")
            .lines()
            .map(|l| String::from(l))
            .collect::<Vec<String>>();

        let o2 = filter(data.clone(), |a, b| a >= b);
        let co2 = filter(data.clone(), |a, b| a < b);

        (o2 * co2).to_string()
    }
);

fn get_most_common(data: &Vec<String>) -> [(usize, usize); 12] {
    let mut bits_counts = [(0, 0); 12];

    for line in data {
        for (i, bit) in line.chars().enumerate() {
            if bit == '1' {
                bits_counts[i].1 += 1;
            } else {
                bits_counts[i].0 += 1
            }
        }
    }

    bits_counts
}

fn filter<C>(mut data: Vec<String>, c: C) -> usize
    where C: Fn(usize, usize) -> bool {
    while data.len() > 1 {
        for i in 0..12 {
            if data.len() == 1 { break; }
            let bits = &get_most_common(&data);
            data = data
                .into_iter()
                .filter(|d| {
                    if c(bits[i].1, bits[i].0) {
                        &d[i..i + 1] == "1"
                    } else {
                        &d[i..i + 1] == "0"
                    }
                })
                .collect::<Vec<String>>();
        }
    }

    usize::from_str_radix(&data[0], 2).unwrap()
}
