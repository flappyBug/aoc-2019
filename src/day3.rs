use std::ops::RangeInclusive;
use Direction::*;

type Point = (i32, i32);

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(PartialEq, Eq, Debug)]
struct DirectionalSegment {
    start_point: Point,
    direction: Direction,
    length: u32,
}

impl DirectionalSegment {
    fn x_range(&self) -> RangeInclusive<i32> {
        let x = self.start_point.0;
        let len = self.length as i32;
        match self.direction {
            Up | Down => x..=x,
            Left => (x - len)..=x,
            Right => x..=(x + len),
        }
    }

    fn y_range(&self) -> RangeInclusive<i32> {
        use Direction::*;
        let y = self.start_point.1;
        let len = self.length as i32;
        match self.direction {
            Left | Right => y..=y,
            Up => y..=(y + len),
            Down => (y - len)..=y,
        }
    }

    fn steps_to(&self, p: Point) -> Option<u32> {
        if self.contains(p) {
            match self.direction {
                Up => Some((p.1 - self.start_point.1) as u32),
                Down => Some((self.start_point.1 - p.1) as u32),
                Left => Some((self.start_point.0 - p.0) as u32),
                Right => Some((p.0 - self.start_point.0) as u32),
            }
        } else {
            None
        }
    }

    fn contains(&self, p: Point) -> bool {
        self.x_range().contains(&p.0) && self.y_range().contains(&p.1)
    }
}

#[derive(Debug)]
pub struct Wire {
    horizontal_segs: Vec<DirectionalSegment>,
    vertical_segs: Vec<DirectionalSegment>,
}

impl Wire {
    fn steps_to(&self, p: Point) -> u32 {
        let horizontal_first =
            !self.horizontal_segs.is_empty() && self.horizontal_segs[0].start_point == (0, 0);
        let mut steps = 0;
        for i in 0.. {
            if horizontal_first {
                let hls = &self.horizontal_segs[i];
                if let Some(last_steps) = hls.steps_to(p) {
                    return steps + last_steps;
                } else {
                    steps += hls.length;
                }
                let vls = &self.vertical_segs[i];
                if let Some(last_steps) = vls.steps_to(p) {
                    return steps + last_steps;
                } else {
                    steps += vls.length;
                }
            } else {
                let vls = &self.vertical_segs[i];
                if let Some(last_steps) = vls.steps_to(p) {
                    return steps + last_steps;
                } else {
                    steps += vls.length;
                }
                let hls = &self.horizontal_segs[i];
                if let Some(last_steps) = hls.steps_to(p) {
                    return steps + last_steps;
                } else {
                    steps += hls.length;
                }
            }
        }
        unreachable!()
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> (Wire, Wire) {
    let mut lines = input.lines();
    (
        parse_line(lines.next().unwrap()),
        parse_line(lines.next().unwrap()),
    )
}

fn parse_line(line: &str) -> Wire {
    let segs = line.split(',');
    let mut horizontal_segs = vec![];
    let mut vertical_segs = vec![];
    let mut cur = (0, 0);
    for seg in segs {
        let (dir, length) = seg.split_at(1);
        let length = length.parse::<u32>().unwrap();
        match dir {
            "R" => {
                horizontal_segs.push(DirectionalSegment {
                    start_point: cur,
                    length,
                    direction: Right,
                });
                cur.0 += length as i32;
            }
            "U" => {
                vertical_segs.push(DirectionalSegment {
                    start_point: cur,
                    length,
                    direction: Up,
                });
                cur.1 += length as i32;
            }
            "L" => {
                horizontal_segs.push(DirectionalSegment {
                    start_point: cur,
                    length,
                    direction: Left,
                });
                cur.0 -= length as i32;
            }
            "D" => {
                vertical_segs.push(DirectionalSegment {
                    start_point: cur,
                    length,
                    direction: Down,
                });
                cur.1 -= length as i32;
            }
            _ => unreachable!(),
        }
    }
    Wire {
        horizontal_segs,
        vertical_segs,
    }
}

fn cross(h: &DirectionalSegment, v: &DirectionalSegment) -> Option<Point> {
    debug_assert!(h.direction == Direction::Left || h.direction == Direction::Right);
    debug_assert!(v.direction == Direction::Up || v.direction == Direction::Down);
    let hy = h.start_point.1;
    let vx = v.start_point.0;

    if h.x_range().contains(&vx) && v.y_range().contains(&hy) {
        Some((vx, hy))
    } else {
        None
    }
}

fn manhattan_distance(p1: Point, p2: Point) -> u32 {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as u32
}

#[aoc(day3, part1)]
pub fn closest_cross((w1, w2): &(Wire, Wire)) -> u32 {
    let mut min_dist = u32::max_value();
    let mut skipped = false;
    for hls in &w1.horizontal_segs {
        for vls in &w2.vertical_segs {
            // skip first iterator because they intersect at origin
            if !skipped {
                skipped = true;
                continue;
            }
            if let Some(p) = cross(hls, vls) {
                min_dist = min_dist.min(manhattan_distance(p, (0, 0)));
            }
        }
    }
    skipped = false;
    for hls in &w2.horizontal_segs {
        for vls in &w1.vertical_segs {
            if !skipped {
                skipped = true;
                continue;
            }
            if let Some(p) = cross(hls, vls) {
                min_dist = min_dist.min(manhattan_distance(p, (0, 0)));
            }
        }
    }
    min_dist
}

#[aoc(day3, part2)]
pub fn min_step((w1, w2): &(Wire, Wire)) -> u32 {
    let mut min_step = u32::max_value();
    let mut skipped = false;
    for hls in &w1.horizontal_segs {
        for vls in &w2.vertical_segs {
            // skip first iterator because they intersect at origin
            if !skipped {
                skipped = true;
                continue;
            }
            if let Some(p) = cross(hls, vls) {
                min_step = min_step.min(w1.steps_to(p) + w2.steps_to(p));
                break;
            }
        }
    }
    skipped = false;
    for hls in &w2.horizontal_segs {
        for vls in &w1.vertical_segs {
            if !skipped {
                skipped = true;
                continue;
            }
            if let Some(p) = cross(hls, vls) {
                min_step = min_step.min(w1.steps_to(p) + w2.steps_to(p));
                break;
            }
        }
    }
    min_step
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    #[test]
    fn test_parse_line() {
        let line = "R1004,D53,L10,U126";
        let wire = parse_line(line);
        let hls = vec![
            DirectionalSegment {
                start_point: (0, 0),
                length: 1004,
                direction: Right,
            },
            DirectionalSegment {
                start_point: (1004, -53),
                length: 10,
                direction: Left,
            },
        ];
        let vls = vec![
            DirectionalSegment {
                start_point: (1004, 0),
                length: 53,
                direction: Down,
            },
            DirectionalSegment {
                start_point: (994, -53),
                length: 126,
                direction: Up,
            },
        ];

        assert_eq!(wire.horizontal_segs, hls);
        assert_eq!(wire.vertical_segs, vls);
    }

    fn test_helper(input: &str, expect_manhattan: u32, expect_steps: u32) {
        let wires = dbg!(parse_input(input));
        assert_eq!(closest_cross(&wires), expect_manhattan);
        assert_eq!(min_step(&wires), expect_steps);
    }
    #[test]
    fn case1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        test_helper(input, 159, 610);
    }
    #[test]
    fn case2() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        test_helper(input, 135, 410);
    }
}
