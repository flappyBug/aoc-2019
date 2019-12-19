use std::collections::{HashMap, HashSet};
#[derive(Debug)]
struct OrbitMap {
    astrals: HashMap<String, Astral>,
}

impl OrbitMap {
    fn add(&mut self, astral1: &str, astral2: &str) {
        self.astrals
            .entry(astral2.to_owned())
            .or_insert(Astral::new(astral2))
            .set_parent(astral1);
        self.astrals
            .entry(astral1.to_owned())
            .or_insert(Astral::new(astral1))
            .add_orbit(astral2);
    }

    fn path_len(&self, start: &str, end: &str) -> usize {
        let mut to_search;
        let mut to_search_next = vec![];
        let mut searched = HashSet::new();
        let mut len = 0;
        let start = self.astrals.get(start).unwrap();
        to_search = start.neighbors();
        while !to_search.is_empty() {
            len += 1;
            to_search_next.clear();
            for astral_name in &to_search {
                if astral_name == end {
                    return len;
                }
                searched.insert(astral_name.to_owned());
                let astral = self.astrals.get(astral_name).unwrap();
                let neighbor_names = astral.neighbors();
                to_search_next.append(
                    &mut neighbor_names
                        .into_iter()
                        .filter(|name| !searched.contains(name))
                        .collect(),
                );
            }
            std::mem::swap(&mut to_search, &mut to_search_next);
        }
        len
    }

    fn count(&self) -> usize {
        let root = self.astrals.get("COM").unwrap();
        let mut cnt = root.orbits.len();
        let mut weight = 1;
        let mut cur = vec![];
        let mut next = vec![];
        cur.append(&mut root.orbits.clone());
        while !cur.is_empty() {
            weight += 1;
            next.clear();
            for name in &cur {
                let astral = self.astrals.get(name).unwrap();
                cnt += astral.orbits.len() * weight;
                next.append(&mut astral.orbits.clone());
            }
            std::mem::swap(&mut cur, &mut next);
        }
        cnt
    }
}
#[derive(Debug)]
struct Astral {
    parent: Option<String>,
    name: String,
    orbits: Vec<String>,
}

impl Astral {
    fn new(name: &str) -> Self {
        Self {
            parent: None,
            name: name.to_owned(),
            orbits: vec![],
        }
    }

    fn set_parent(&mut self, parent: &str) {
        self.parent = Some(parent.to_owned());
    }

    fn add_orbit(&mut self, orbit_name: &str) {
        self.orbits.push(orbit_name.to_owned());
    }

    fn neighbors(&self) -> Vec<String> {
        let mut v = self.orbits.clone();
        if let Some(ref parent) = self.parent {
            v.push(parent.to_owned());
        }
        v
    }
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> OrbitMap {
    let mut map = OrbitMap {
        astrals: HashMap::new(),
    };
    input.lines().for_each(|line| {
        let mut astrals = line.split(')');
        let astral1 = astrals.next().unwrap();
        let astral2 = astrals.next().unwrap();
        map.add(astral1, astral2);
    });
    map
}

#[aoc(day6, part1)]
fn part1(orbit_map: &OrbitMap) -> usize {
    orbit_map.count()
}

#[aoc(day6, part2)]
fn part2(orbit_map: &OrbitMap) -> usize {
    orbit_map.path_len("YOU", "SAN") - 2
}
