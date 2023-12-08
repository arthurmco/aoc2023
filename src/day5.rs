#![allow(dead_code, unused_mut, unused_variables)]

use crate::util::{read_file_as_text, split_numbers_by_space};
use std::io::prelude::*;

type SeedRange = (usize, usize);

#[derive(Debug, Clone)]
struct AlmanacRange {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

impl AlmanacRange {
    fn from_line(l: &str) -> AlmanacRange {
        let mut elements = l.split(' ');
        AlmanacRange {
            destination_start: elements.next().unwrap().parse().unwrap(),
            source_start: elements.next().unwrap().parse().unwrap(),
            range_length: elements.next().unwrap().parse().unwrap(),
        }
    }

    fn correspondences(ranges: &[AlmanacRange], source: usize) -> usize {
        ranges
            .iter()
            .find(|r| {
                let end = r.source_start + r.range_length;
                r.source_start <= source && source < end
            })
            .map(|r| {
                let offset = source - r.source_start;
                r.destination_start + offset
            })
            .unwrap_or(source)
    }

    fn correspondence_ranges(ranges: &[AlmanacRange], source: SeedRange) -> Vec<SeedRange> {
        let mut ranges_in_source: Vec<&AlmanacRange> = ranges
            .iter()
            .filter(|r| {
                let (sstart, slen) = source;
                let range_end = r.source_start + r.range_length;

                (r.source_start.saturating_sub(r.range_length)) <= sstart && (sstart + slen) <= (range_end)
            })
            .collect();

        if ranges_in_source.is_empty() {
            vec![source]
        } else {
            vec![source]
            /*ranges_in_source.sort_by(|a, b| a.source_start.cmp(&b.source_start));
            //println!("\n\n\n\n\n\n\nris {:?} {:?}", ranges_in_source, source);
            
            let mut ret = vec![];
            let mut current = source.0;
            let end = current + source.1;

            eprintln!("-");
            for range in ranges_in_source {
                if current < range.source_start {
                    let len = (range.source_start - current).min(end-current);
                    ret.push((current, len));
                    current += len;
                }                

                if current >= end {
                    //eprintln!("{} ret {:?}", current, ret);
                    break;
                }                         

                let from_start = range.source_start;
                let to_start = range.destination_start;
                let from_end = from_start + range.range_length;
                let offset = current - range.source_start;
                let real_start = from_start + offset;               

                //eprintln!("{} - {}", end, current);
                let remaining_len = end - current;

                let length = (remaining_len).min(from_end - real_start);
                //println!("{} = min({}, {} - {}) {}", length, remaining_len,
                //from_end, real_start, from_end-real_start);
                
                //                println!("fe {} {:?} {}, {}, {}", length, ((to_start+offset, length)), end, current,
                //         end.saturating_sub(current));

                
                //eprintln!("r {:?}", range);
                ret.push((to_start + offset, length));
                //eprintln!("{} ret {:?}", current, ret);

                current += length;

                if current >= end {
                    break;
                }                               
            }
            
            assert!(current <= end);
            if current < end {
                ret.push((current, end - current));
            } 
            
            ret
          */  
        } 
    }
}

#[derive(Debug)]
struct SeedFile {
    initial_seeds: Vec<SeedRange>,
    //initial_seeds: Vec<usize>, p1
    seed_to_soil: Vec<AlmanacRange>,
    soil_to_fertilizer: Vec<AlmanacRange>,
    fertilizer_to_water: Vec<AlmanacRange>,
    water_to_light: Vec<AlmanacRange>,
    light_to_temperature: Vec<AlmanacRange>,
    temperature_to_humidity: Vec<AlmanacRange>,
    humidity_to_location: Vec<AlmanacRange>,
}

enum SeedFileParseState {
    NoState,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl SeedFile {
    /* fn initial_seeds(&self) -> &[usize] {
        &self.initial_seeds
    } */

    fn initial_seeds(&self) -> &[(usize, usize)] {
        &self.initial_seeds
    }

    fn seed_to_location(&self, seed: usize) -> usize {
        let soil = AlmanacRange::correspondences(&self.seed_to_soil, seed);
        let fertilizer = AlmanacRange::correspondences(&self.soil_to_fertilizer, soil);
        let water = AlmanacRange::correspondences(&self.fertilizer_to_water, fertilizer);
        let light = AlmanacRange::correspondences(&self.water_to_light, water);
        let temperature = AlmanacRange::correspondences(&self.light_to_temperature, light);
        let humidity = AlmanacRange::correspondences(&self.temperature_to_humidity, temperature);
        

        /* eprintln!(
            "soil {} fert {} water {} light {} temp {} humidity {}",
            soil, fertilizer, water, light, temperature, humidity
        ); */

        AlmanacRange::correspondences(&self.humidity_to_location, humidity)
    }

    fn seed_to_location_ranges(&self, seed: SeedRange) -> Vec<SeedRange> {
        let soil = AlmanacRange::correspondence_ranges(&self.seed_to_soil, seed);
        let fertilizer: Vec<SeedRange> = soil
            .iter()
            .flat_map(|s| AlmanacRange::correspondence_ranges(&self.soil_to_fertilizer, *s))
            .collect();
        let water: Vec<SeedRange> = fertilizer
            .iter()
            .flat_map(|s| AlmanacRange::correspondence_ranges(&self.fertilizer_to_water, *s))
            .collect();
        let light: Vec<SeedRange> = water
            .iter()
            .flat_map(|s| AlmanacRange::correspondence_ranges(&self.water_to_light, *s))
            .collect();
        let temperature: Vec<SeedRange> = light
            .iter()
            .flat_map(|s| {
                AlmanacRange::correspondence_ranges(&self.light_to_temperature, *s)
            })
            .collect();
        let humidity: Vec<SeedRange> = temperature
            .iter()
            .flat_map(|s| {
                AlmanacRange::correspondence_ranges(&self.temperature_to_humidity, *s)
            })
            .collect();
        let location: Vec<SeedRange> = humidity
            .iter()
            .flat_map(|s| {
                AlmanacRange::correspondence_ranges(&self.humidity_to_location, *s)
            })
            .collect();

        eprintln!(
            "\tseed: {:?}\n\tsoil {:?} \n\tfert {:?} \n\twater {:?} \n\tlight {:?} \n\ttemp {:?} \n\thumidity {:?}\n\tlocation: {:?}",
            seed, soil, fertilizer, water, light, temperature, humidity, location
        );

        location
    }

    fn finish_state(
        initial_seed: SeedFile,
        current_vec: &Vec<AlmanacRange>,
        current_state: SeedFileParseState,
    ) -> SeedFile {
        match current_state {
            SeedFileParseState::NoState => initial_seed,
            SeedFileParseState::SeedToSoil => SeedFile {
                seed_to_soil: current_vec.clone(),
                ..initial_seed
            },
            SeedFileParseState::SoilToFertilizer => SeedFile {
                soil_to_fertilizer: current_vec.clone(),
                ..initial_seed
            },
            SeedFileParseState::FertilizerToWater => SeedFile {
                fertilizer_to_water: current_vec.clone(),
                ..initial_seed
            },
            SeedFileParseState::WaterToLight => SeedFile {
                water_to_light: current_vec.clone(),
                ..initial_seed
            },
            SeedFileParseState::LightToTemperature => SeedFile {
                light_to_temperature: current_vec.clone(),
                ..initial_seed
            },
            SeedFileParseState::TemperatureToHumidity => SeedFile {
                temperature_to_humidity: current_vec.clone(),
                ..initial_seed
            },
            SeedFileParseState::HumidityToLocation => SeedFile {
                humidity_to_location: current_vec.clone(),
                ..initial_seed
            },
        }
    }

    fn split_initial_seeds(line: &str) -> Vec<(usize, usize)> {
        split_numbers_by_space(line)
            .chunks(2)
            .map(|v| (v[0], v[1]))
            .collect()
    }

    fn from_lines(mut lines: impl Iterator<Item = String>) -> SeedFile {
        let seed_line = lines.next().unwrap();
        let initial_seeds = SeedFile::split_initial_seeds(&seed_line[6..]);
        //let initial_seeds = split_numbers_by_space(&seed_line[6..]);

        let mut initial_seed = SeedFile {
            initial_seeds,
            seed_to_soil: vec![],
            soil_to_fertilizer: vec![],
            fertilizer_to_water: vec![],
            water_to_light: vec![],
            light_to_temperature: vec![],
            temperature_to_humidity: vec![],
            humidity_to_location: vec![],
        };

        let mut current_state: SeedFileParseState = SeedFileParseState::NoState;
        let mut current_vec: Vec<AlmanacRange> = vec![];
        for line in lines {
            match line.as_str() {
                "seed-to-soil map:" => {
                    current_state = SeedFileParseState::SeedToSoil;
                }
                "soil-to-fertilizer map:" => {
                    current_state = SeedFileParseState::SoilToFertilizer;
                }
                "fertilizer-to-water map:" => {
                    current_state = SeedFileParseState::FertilizerToWater;
                }
                "water-to-light map:" => {
                    current_state = SeedFileParseState::WaterToLight;
                }
                "light-to-temperature map:" => {
                    current_state = SeedFileParseState::LightToTemperature;
                }
                "temperature-to-humidity map:" => {
                    current_state = SeedFileParseState::TemperatureToHumidity;
                }
                "humidity-to-location map:" => {
                    current_state = SeedFileParseState::HumidityToLocation;
                }
                l if l.trim() == "" => {
                    // end of map
                    initial_seed =
                        SeedFile::finish_state(initial_seed, &current_vec, current_state);
                    current_state = SeedFileParseState::NoState;
                    current_vec.clear();
                }
                l if !l.contains("map:") => {
                    // vector list
                    current_vec.push(AlmanacRange::from_line(&line))
                }
                _ => {}
            }
        }

        SeedFile::finish_state(initial_seed, &current_vec, current_state)
    }
}

/*
pub fn day5() {
    //let game_file = read_file_as_text("./inputs/day5real.txt").lines();
    let game_file = read_file_as_text("./inputs/day5test.txt").lines();
    let seed_file = SeedFile::from_lines(game_file.map(|l| l.unwrap()));

    //let ranges = vec![AlmanacRange::from_line("50 98 2"), AlmanacRange::from_line("52 50 48")];
    //println!("Hello {:?} ", seed_file);

    let min_location = seed_file
        .initial_seeds()
        .iter()
        .map(|s| seed_file.seed_to_location(*s))
        .inspect(|s| eprintln!("seed {}", s))
        .min()
        .unwrap();

println!("\n{}", min_location);
}
 */
// 198106515: too high
// 26714516: certa

/*
pub fn day5() {
    //let game_file = read_file_as_text("./inputs/day5real.txt").lines();
    let game_file = read_file_as_text("./inputs/day5real.txt").lines();
    let seed_file = SeedFile::from_lines(game_file.map(|l| l.unwrap()));

    //let ranges = vec![AlmanacRange::from_line("50 98 2"), AlmanacRange::from_line("52 50 48")];
    //println!("Hello {:?} ", seed_file);

    let min_location = seed_file
        .initial_seeds()
        .iter()
        .flat_map(|&(ss,sl)| ss..(ss+sl))
        .map(|s| seed_file.seed_to_location(s))
        //.inspect(|s| eprintln!("seed {}", s))
        .min()
        .unwrap();

    println!("\n{}", min_location);
}
*/

pub fn day5() {
    let game_file = read_file_as_text("./inputs/day5real.txt").lines();
    //let game_file = read_file_as_text("./inputs/day5test.txt").lines();
    let seed_file = SeedFile::from_lines(game_file.map(|l| l.unwrap()));
    
    //let ranges = vec![AlmanacRange::from_line("50 98 2"), AlmanacRange::from_line("52 50 48")];
    println!("Hello {:?} ", seed_file);
    let locations = seed_file
        .initial_seeds() // &[(3082872446, 316680412)]
        .iter()
        .flat_map(|s| seed_file.seed_to_location_ranges(*s))
        //.inspect(|s| eprintln!("ranges {:?}", s))
        .map(|(rs, _)| rs)
        .min()
        .unwrap();

    println!("\n{}", locations)

}

