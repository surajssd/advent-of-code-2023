// Here is the source: https://adventofcode.com/2023/day/5
// Here is the sample input: https://adventofcode.com/2023/day/5/input

use std::collections::HashMap;
use std::fs;

const INPUT_FILE: &str = "input";
// const INPUT_FILE: &str = "smallinput";

fn main() {
    let lines: Vec<String> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let n = Input::new(lines);

    let mut loc: Vec<u32> = Vec::new();

    let mut idx: usize = 0;
    let mut smallest_lhs: u32 = 0;
    let mut smallest_rhs: u32 = 0;
    for (k, v) in n.humidity_to_location.iter() {
        small = n.humidity_to_location[k].destination;
        break;
    }

    while idx < n.seeds.len() {
        let seed = n.seeds[idx];
        idx += 1;
        let count = n.seeds[idx];
        idx += 1;

        for offset in 0..count {
            let new_seed = seed + offset;
            let s = n.get_seed_to_soil(new_seed);
            let f = n.get_soil_to_fertilizer(s);
            let w = n.get_fertilizer_to_water(f);
            let l = n.get_water_to_light(w);
            let t = n.get_light_to_temperature(l);
            let h = n.get_temperature_to_humidity(t);
            let l = n.get_humidity_to_location(h);

            // if small > l {
            //     small = l;
            // }
        }
    }

    // Find the smallest location.
    // let mut small = loc[0];
    // for l in loc.iter() {
    //     if small > *l {
    //         small = *l;
    //     }
    // }

    println!("{}", small);
}

#[derive(Debug, Clone)]
struct Input {
    seeds: Vec<u32>,
    seed_to_soil: HashMap<u32, Map>,
    soil_to_fertilizer: HashMap<u32, Map>,
    fertilizer_to_water: HashMap<u32, Map>,
    water_to_light: HashMap<u32, Map>,
    light_to_temperature: HashMap<u32, Map>,
    temperature_to_humidity: HashMap<u32, Map>,
    humidity_to_location: HashMap<u32, Map>,
}

#[derive(Debug, Copy, Clone)]
struct Map {
    destination: u32,
    source: u32,
    range: u32,
}

impl Map {
    fn in_range(&self, n: u32) -> bool {
        self.source <= n && n < self.source + self.range
    }

    fn corresponding_dest(&self, n: u32) -> u32 {
        self.destination + (n - self.source)
    }
}

fn map_src_dest(h: &HashMap<u32, Map>, s: u32) -> u32 {
    for (_, v) in h.iter() {
        if v.in_range(s) {
            return v.corresponding_dest(s);
        }
    }

    s
}

impl Input {
    fn get_seed_to_soil(&self, seed: u32) -> u32 {
        map_src_dest(&self.seed_to_soil, seed)
    }

    fn get_soil_to_fertilizer(&self, seed: u32) -> u32 {
        map_src_dest(&self.soil_to_fertilizer, seed)
    }

    fn get_fertilizer_to_water(&self, seed: u32) -> u32 {
        map_src_dest(&self.fertilizer_to_water, seed)
    }

    fn get_water_to_light(&self, seed: u32) -> u32 {
        map_src_dest(&self.water_to_light, seed)
    }

    fn get_light_to_temperature(&self, seed: u32) -> u32 {
        map_src_dest(&self.light_to_temperature, seed)
    }

    fn get_temperature_to_humidity(&self, seed: u32) -> u32 {
        map_src_dest(&self.temperature_to_humidity, seed)
    }

    fn get_humidity_to_location(&self, seed: u32) -> u32 {
        map_src_dest(&self.humidity_to_location, seed)
    }

    fn new(lines: Vec<String>) -> Self {
        let mut idx = 0;
        let mut numbers: Vec<u32> = Vec::new();
        let mut seed_to_soil: HashMap<u32, Map> = HashMap::new();
        let mut soil_to_fertilizer: HashMap<u32, Map> = HashMap::new();
        let mut fertilizer_to_water: HashMap<u32, Map> = HashMap::new();
        let mut water_to_light: HashMap<u32, Map> = HashMap::new();
        let mut light_to_temperature: HashMap<u32, Map> = HashMap::new();
        let mut temperature_to_humidity: HashMap<u32, Map> = HashMap::new();
        let mut humidity_to_location: HashMap<u32, Map> = HashMap::new();

        while idx < lines.len() {
            let line = &lines[idx];

            // Ignore empty lines
            if line == "" {
                idx += 1;
                continue;
            }

            if line.contains("seeds") {
                let seeds_and_numbers: Vec<&str> = line.split(":").collect();
                numbers = seeds_and_numbers[1]
                    .trim()
                    .split(" ")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();

                idx += 1;
                continue;
            }

            if line.contains("seed-to-soil map:") {
                idx += 1;
                (seed_to_soil, idx) = Self::parse_maps(&lines, idx);
            }

            if line.contains("soil-to-fertilizer map:") {
                idx += 1;
                (soil_to_fertilizer, idx) = Self::parse_maps(&lines, idx);
            }

            if line.contains("fertilizer-to-water map:") {
                idx += 1;
                (fertilizer_to_water, idx) = Self::parse_maps(&lines, idx);
            }

            if line.contains("water-to-light map:") {
                idx += 1;
                (water_to_light, idx) = Self::parse_maps(&lines, idx);
            }

            if line.contains("light-to-temperature map:") {
                idx += 1;
                (light_to_temperature, idx) = Self::parse_maps(&lines, idx);
            }

            if line.contains("temperature-to-humidity map:") {
                idx += 1;
                (temperature_to_humidity, idx) = Self::parse_maps(&lines, idx);
            }

            if line.contains("humidity-to-location map:") {
                idx += 1;
                (humidity_to_location, idx) = Self::parse_maps(&lines, idx);
            }

            idx += 1;
        }

        Self {
            seeds: numbers,
            seed_to_soil: seed_to_soil,
            soil_to_fertilizer: soil_to_fertilizer,
            fertilizer_to_water: fertilizer_to_water,
            water_to_light: water_to_light,
            light_to_temperature: light_to_temperature,
            temperature_to_humidity: temperature_to_humidity,
            humidity_to_location: humidity_to_location,
        }
    }

    fn parse_maps(lines: &Vec<String>, mut idx: usize) -> (HashMap<u32, Map>, usize) {
        let mut ret: HashMap<u32, Map> = HashMap::new();

        while idx < lines.len() && lines[idx] != "" {
            let dest_src_range: Vec<u32> = lines[idx]
                .split(" ")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            let m = Map {
                destination: dest_src_range[0],
                source: dest_src_range[1],
                range: dest_src_range[2],
            };

            ret.insert(m.source, m);
            idx += 1;
        }

        (ret, idx)
    }
}
