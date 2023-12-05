pub mod seeds {
    use std::cmp::Ordering;
    use std::collections::{BTreeSet, HashMap};

    use itertools::Itertools;

    #[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
    pub struct SeedMapping {
        source: u32,
        destination: u32,
        range: u32,
    }

    impl SeedMapping {
        pub fn parse(string: &str) -> Self {
            let (destination, source, range) = string.split_whitespace()
                .map(|str| str.parse().unwrap())
                .collect_tuple::<(u32, u32, u32)>()
                .unwrap();
            Self { source, destination, range }
        }

        pub fn end(&self) -> u128 {
            self.source as u128 + self.range as u128
        }

        pub fn in_range(&self, value: u32) -> bool {
            value >= self.source && (value as u128) < self.end()
        }

        pub fn convert(&self, value: u32) -> u32 {
            return (value as i128 - self.source as i128 + self.destination as i128) as u32;
        }

        pub fn find_overlap(&self, range: &SeedRange) -> (Option<SeedRange>, Option<SeedRange>, Option<SeedRange>) {
            let range_end = range.0 as u128 + range.1 as u128;
            let self_end = self.end();

            // No overlap cases
            if range_end <= (self.source as u128) {
                // Entire range is to the left of self
                return (Some(*range), None, None);
            }

            if (range.0 as u128) >= self_end {
                // Entire range is to the right of self
                return (None, None, Some(*range));
            }

            // Overlap cases
            let left = if range.0 < self.source {
                Some(SeedRange(range.0, self.source - range.0))
            } else {
                None
            };

            let overlap_start = std::cmp::max(self.source, range.0) as u128;
            let overlap_end = std::cmp::min(self_end, range_end);
            let overlap = Some(SeedRange(overlap_start as u32, (overlap_end - overlap_start) as u32));

            let right = if range_end > self_end {
                Some(SeedRange(self_end as u32, (range_end - self_end) as u32))
            } else {
                None
            };

            (left, overlap, right)
        }
    }

    impl PartialOrd for SeedMapping {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.source.partial_cmp(&other.source)
        }
    }

    impl Ord for SeedMapping {
        fn cmp(&self, other: &Self) -> Ordering {
            self.source.cmp(&other.source)
        }
    }

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    pub struct SeedRange(u32, u32);

    impl SeedRange {
        pub fn new(start: u32, range: u32) -> Self { Self(start, range) }

        pub fn with_mapping(&self, mapping: &SeedMapping) -> Self {
            Self(mapping.convert(self.0), self.1)
        }

        pub fn lowest(&self) -> u32 {
            self.0
        }
    }

    impl PartialOrd for SeedRange {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for SeedRange {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    #[derive(Debug)]
    pub struct SeedMap {
        destination: String,
        mappings: BTreeSet<SeedMapping>,
    }

    impl SeedMap {
        pub fn new(destination: String) -> Self {
            Self {
                destination,
                mappings: BTreeSet::new(),
            }
        }

        pub fn add_mapping(&mut self, mapping: SeedMapping) {
            self.mappings.insert(mapping);
        }

        pub fn convert(&self, value: u32) -> u32 {
            for mapping in &self.mappings {
                if mapping.in_range(value) {
                    return mapping.convert(value);
                }
            }
            value
        }

        pub fn convert_range(&self, range: &SeedRange) -> BTreeSet<SeedRange> {
            let mut set = BTreeSet::new();
            let mut current_range = range.clone();
            let mut resting_range = true;

            for mapping in &self.mappings {
                let (left, overlap, right) = mapping.find_overlap(&current_range);

                if let Some(left) = left {
                    set.insert(left); // To the left of all ranges
                }

                if let Some(overlap) = overlap {
                    set.insert(overlap.with_mapping(mapping));
                }

                if let Some(right) = right {
                    current_range = right;
                } else {
                    resting_range = false;
                    break;
                }
            }

            if resting_range {
                set.insert(current_range);
            }

            set
        }
    }

    #[derive(Debug)]
    pub struct SeedMaps(HashMap<String, SeedMap>);

    impl SeedMaps {
        pub fn new() -> Self {
            Self(HashMap::new())
        }

        pub fn new_map(&mut self, source: &str, destination: &str) -> String {
            let map = SeedMap::new(destination.to_string());
            let key = source.to_string();

            self.0.insert(source.to_string(), map);

            key
        }

        pub fn get_mut_map(&mut self, source: &str) -> &mut SeedMap {
            self.0.get_mut(source).unwrap()
        }

        pub fn get_value(&self, value: u32, source_type: &str, value_type: &str) -> Option<u32> {
            if source_type == value_type {
                return Some(value);
            }

            if let Some(map) = self.0.get(source_type) {
                let next_value = map.convert(value);

                self.get_value(next_value, map.destination.as_str(), value_type)
            } else {
                None
            }
        }

        pub fn get_value_for_ranges(&self, pairs: BTreeSet<SeedRange>, source_type: &str, value_type: &str) -> Option<BTreeSet<SeedRange>> {
            if source_type == value_type {
                return Some(pairs);
            }

            if let Some(map) = self.0.get(source_type) {
                let new_pairs: BTreeSet<SeedRange> = pairs.iter()
                    .flat_map(|range| map.convert_range(range))
                    .collect();

                self.get_value_for_ranges(new_pairs, map.destination.as_str(), value_type)
            } else {
                None
            }
        }
    }
}
