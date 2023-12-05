// https://adventofcode.com/2023/day/5
enum TransformType {
    SeedToSoil,
    SoilToFert,
    FertToWater,
    WaterToLight,
    LightToTemp,
    TempToHum,
    HumToLoc,
    None,
}

fn main() {
    let contents = include_str!("../input.txt");
    println!("PART1: {}", part1(contents));
}

fn part1(content: &str) -> u64 {
    let mut kind: TransformType = TransformType::None;
    let mut seeds_to_plant: Vec<u64> = vec![];

    // This really does look dumb
    let mut seed_soil: Vec<(u64, u64, u64)> = vec![];
    let mut soil_fert: Vec<(u64, u64, u64)> = vec![];
    let mut fert_water: Vec<(u64, u64, u64)> = vec![];
    let mut water_light: Vec<(u64, u64, u64)> = vec![];
    let mut light_temp: Vec<(u64, u64, u64)> = vec![];
    let mut temp_humid: Vec<(u64, u64, u64)> = vec![];
    let mut humid_loca: Vec<(u64, u64, u64)> = vec![];

    content.lines().for_each(|line| match line {
        "" => {}
        "seed-to-soil map:" => kind = TransformType::SeedToSoil,
        "soil-to-fertilizer map:" => kind = TransformType::SoilToFert,
        "fertilizer-to-water map:" => kind = TransformType::FertToWater,
        "water-to-light map:" => kind = TransformType::WaterToLight,
        "light-to-temperature map:" => kind = TransformType::LightToTemp,
        "temperature-to-humidity map:" => kind = TransformType::TempToHum,
        "humidity-to-location map:" => kind = TransformType::HumToLoc,
        // Nums
        _ => {
            if line.starts_with("seeds:") {
                line.split(' ')
                    .map(parse_num)
                    .filter(|s| s.is_some())
                    .for_each(|s| seeds_to_plant.push(s.unwrap()));
            } else {
                // Yes, this is dumb
                let nums: Vec<u64> = line
                    .split(' ')
                    .map(parse_num)
                    .filter(|s| s.is_some())
                    .flatten()
                    .collect();

                match kind {
                    // 0 is dest, 1 src, 2 len
                    TransformType::SeedToSoil => seed_soil.push((nums[1], nums[0], nums[2])),
                    TransformType::SoilToFert => soil_fert.push((nums[1], nums[0], nums[2])),
                    TransformType::FertToWater => fert_water.push((nums[1], nums[0], nums[2])),
                    TransformType::WaterToLight => water_light.push((nums[1], nums[0], nums[2])),
                    TransformType::LightToTemp => light_temp.push((nums[1], nums[0], nums[2])),
                    TransformType::TempToHum => temp_humid.push((nums[1], nums[0], nums[2])),
                    TransformType::HumToLoc => humid_loca.push((nums[1], nums[0], nums[2])),
                    TransformType::None => unreachable!("Should not happen!"),
                }
            }
        }
    });

    seed_soil.sort_by(|(a, _, _), (d, _, _)| a.cmp(d));
    soil_fert.sort_by(|(a, _, _), (d, _, _)| a.cmp(d));
    fert_water.sort_by(|(a, _, _), (d, _, _)| a.cmp(d));
    water_light.sort_by(|(a, _, _), (d, _, _)| a.cmp(d));
    light_temp.sort_by(|(a, _, _), (d, _, _)| a.cmp(d));
    temp_humid.sort_by(|(a, _, _), (d, _, _)| a.cmp(d));
    humid_loca.sort_by(|(a, _, _), (d, _, _)| a.cmp(d));

    let mut min_loc = u64::MAX;
    for seed in &seeds_to_plant {
        let soil_id = map_values(seed, &seed_soil);
        let fert_id = map_values(&soil_id, &soil_fert);
        let water_id = map_values(&fert_id, &fert_water);
        let light_id = map_values(&water_id, &water_light);
        let temp_id = map_values(&light_id, &light_temp);
        let humid_id = map_values(&temp_id, &temp_humid);
        let location_id = map_values(&humid_id, &humid_loca);

        //println!(
        //    "{} -> {} -> {} -> {} -> {} -> {} -> {} -> {}",
        //    seed, soil_id, fert_id, water_id, light_id, temp_id, humid_id, location_id
        //);

        min_loc = min_loc.min(location_id);
    }

    min_loc
}

fn map_values(seed: &u64, seed_soil: &Vec<(u64, u64, u64)>) -> u64 {
    match binary_search_ranges(*seed, seed_soil) {
        Some((src, dest)) => {
            if src == *seed {
                dest
            } else if dest > src {
                seed + dest - src
            } else {
                seed - (src - dest)
            }
        }
        _ => *seed,
    }
}

// I don't think there are enough values in any map to make this make worth
fn binary_search_ranges(seed: u64, map: &Vec<(u64, u64, u64)>) -> Option<(u64, u64)> {
    let mut lo = 0_i32;
    let mut hi = map.len() as i32 - 1;

    while lo <= hi {
        let m = (lo + hi) / 2;

        let tuple = map[m as usize];
        if tuple.0 < seed && tuple.0 + tuple.2 > seed || tuple.0 == seed {
            return Some((tuple.0, tuple.1));
        } else if seed > tuple.0 {
            lo = m + 1;
        } else if seed < tuple.0 {
            hi = m - 1;
        }
    }

    None
}

// Same function with same problems
fn parse_num(num: &str) -> Option<u64> {
    let mut num_buff: u64 = 0;

    let mut pos = num.len() as u32;
    for ch in num.chars() {
        if let '0'..='9' = ch {
            let n = ch as u8 - b'0';
            num_buff += n as u64 * (10_u64).pow(pos - 1);
            pos -= 1;
        } else {
            return None;
        }
    }

    Some(num_buff)
}
