solution!(
    "Day 3: Binary Diagnostic",
    || {
        let mut bits_counts = [(0, 0); 12];
        let mut gamma = String::from("");

        for datum in common::load("day_03").lines() {
            for (i, bit) in datum.chars().enumerate() {
                if bit == '1' {
                    bits_counts[i].1 += 1;
                } else {
                    bits_counts[i].0 += 1
                }
            }
        }

        for counts in bits_counts {
            if counts.1 > counts.0 {
                gamma.push('1');
            } else {
                gamma.push('0');
            }
        }

        let gamma = u16::from_str_radix(&gamma, 2).unwrap();
        let epsilon = ! gamma & 0b0000111111111111;

        (gamma as usize * epsilon as usize).to_string()
    }
);
