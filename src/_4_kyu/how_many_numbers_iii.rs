/*
 * https://www.codewars.com/kata/5877e7d568909e5ff90017e6
 */


use std::cmp::min;
use std::ops::Add;


fn sol1(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let mut valid_nums: Vec<i32> = vec![];

    let start = 10_i64.pow((digs - 1) as u32);
    let end = 10_i64.pow(digs as u32);

    'outer:
    for i in start..end {
        let mut previous = '0';
        let mut sum_of_digits = 0;

        for c in i.to_string().chars() {
            if c < previous { continue 'outer; }
            previous = c;

            sum_of_digits += c.to_digit(10).unwrap();
        }

        if sum_of_digits == sum_dig as u32 {
            valid_nums.push(i as i32);
        }
    }

    valid_nums.sort();

    match valid_nums.len() {
        0 => { None }
        n => { Some((n, *valid_nums.first().unwrap() as u64, *valid_nums.last().unwrap() as u64)) }
    }
}

fn sol2(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let mut count = 0;

    let mut first = String::new();

    let mut sum_temp = sum_dig;
    let mut digs_temp = digs;

    for _ in 0..digs {
        let digit = min(9, sum_temp - digs_temp + 1);
        first = digit.to_string() + &*first;
        sum_temp -= digit;
        digs_temp -= 1;
    }

    let last = (sum_dig / digs).to_string().repeat(digs as usize).parse::<u64>().unwrap() + (sum_dig % digs) as u64;

    let mut i: u64 = first.parse().unwrap();

    while i <= last {
        let digits: Vec<u64> = i.to_string().chars().map(|digit| digit.to_digit(10).unwrap() as u64).collect();

        let sum = digits.iter().sum::<u64>();

        if sum > sum_dig as u64 {
            i += 8;
            continue;
        } else if sum < sum_dig as u64 {
            i += sum_dig as u64 - sum;
            continue;
        }

        let mut sorted_digits = digits.clone();
        sorted_digits.sort();

        if digits == sorted_digits {
            count += 1;
        }

        i += 1;
    }

    Some((count, first.parse().unwrap(), last as u64))
}

fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let nums: Vec<Vec<u8>> = get_ascending_digits(digs, 1).into_iter()
        .filter(
            |vec| vec.iter().sum::<u8>() == sum_dig
        ).collect();

    return match nums.len() {
        0 => None,
        _ => Some((nums.len(), digit_vec_to_num(nums.first().unwrap()), digit_vec_to_num(nums.last().unwrap())))
    }
}

fn digit_vec_to_num(digits: &Vec<u8>) -> u64 {
    digits.iter().fold(String::new(), |acc, d| acc.add(&*d.to_string())).parse().unwrap()
}

fn get_ascending_digits(digs: u8, minimum: u8) -> Vec<Vec<u8>> {
    let mut result = vec![];

    if digs == 1 {
        for x in minimum..10 {
            result.push(vec![x]);
        }
    } else {
        for x in minimum..10 {
            let vec1 = vec![x];
            for mut y in get_ascending_digits(digs - 1, x) {
                let mut vecc = vec1.clone();
                vecc.append(&mut y);
                result.push(vecc)
            }
        }
    }

    result
}



// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::{find_all, sol1, sol2};

    #[test]
    fn sample_tests() {
        assert_eq!(find_all(10, 3), Some((8, 118, 334)));
        assert_eq!(find_all(27, 3), Some((1, 999, 999)));
        assert_eq!(find_all(84, 4), None);
        assert_eq!(find_all(35, 6), Some((123, 116999, 566666)));
    }
    
    #[test]
    fn performance_tests() {
        let mut start = Instant::now();
        println!("{:?}", sol1(13, 3));  // Some((8, 118, 334)), because [118, 127, 136, 145, 226, 235, 244, 334]
        println!("{:?}", start.elapsed());

        start = Instant::now();
        println!("{:?}", sol2(13, 3));
        println!("{:?}", start.elapsed());

        start = Instant::now();
        println!("{:?}", find_all(13, 3));
        println!("{:?}\n\n", start.elapsed());



        start = Instant::now();
        println!("{:?}", sol1(13, 6));
        println!("{:?}", start.elapsed());

        start = Instant::now();
        println!("{:?}", sol2(13, 6));
        println!("{:?}", start.elapsed());

        start = Instant::now();
        println!("{:?}", find_all(13, 6));
        println!("{:?}\n\n", start.elapsed());



        start = Instant::now();
        println!("{:?}", sol2(13, 8));
        println!("{:?}", start.elapsed());

        start = Instant::now();
        println!("{:?}", find_all(13, 8));
        println!("{:?}\n\n", start.elapsed());
    }
}

