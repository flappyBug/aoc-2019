use std::ops::RangeInclusive;
fn two_same_adjacency(num: u32) -> bool {
    let mut num = num;
    while num > 0 {
        if num % 10 == (num / 10) % 10 {
            return true;
        }
        num /= 10;
    }
    false
}

fn two_same_adjacency_strict(num: u32) -> bool {
    let mut num = num;
    while num > 0 {
        let mut same_count = 1;
        while num % 10 == (num / 10) % 10 {
            same_count += 1;
            num /= 10;
        }
        if same_count == 2 {
            return true;
        }
        num /= 10;
    }
    false
}

fn never_decrease(num: u32) -> bool {
    let mut num = num;
    while num > 0 {
        if num % 10 < (num / 10) % 10 {
            return false;
        }
        num /= 10;
    }
    true
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> RangeInclusive<u32> {
    let mut range = input.split('-').map(|n| n.parse::<u32>().unwrap());
    range.next().unwrap()..=range.next().unwrap()
}

#[aoc(day4, part1)]
fn suitable_count(range: &RangeInclusive<u32>) -> usize {
    range
        .clone()
        .filter(|n| never_decrease(*n) && two_same_adjacency(*n))
        .count()
}
#[aoc(day4, part2)]
fn suitable_count_strict(range: &RangeInclusive<u32>) -> usize {
    range
        .clone()
        .filter(|n| never_decrease(*n) && two_same_adjacency_strict(*n))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_same_adjacency() {
        assert!(two_same_adjacency(12234));
        assert!(!two_same_adjacency(1234));
    }
}
