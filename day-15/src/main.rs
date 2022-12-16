use inpt::Inpt;
use std::cmp;
use std::ops::Range;

#[derive(Inpt, Debug, Copy, Clone)]
#[inpt(regex = r"x=(-?\d+),\s+y=(-?\d+)")]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Inpt, Debug)]
#[inpt(regex = r"Sensor at ([^:]+): closest beacon is at\s+")]
struct Sensor {
    location: Point,
    #[inpt(after)]
    closest_beacon: Point,
}

#[derive(Debug)]
struct Ranges {
    ranges: Vec<Range<i64>>,
}

//const MAX_X: i64 = 20;
//const MAX_Y: i64 = 20;
const MAX_X: i64 = 4000000;
const MAX_Y: i64 = 4000000;

#[inpt::main]
fn main(sensors: Vec<Sensor>) {
    let (x, y) = find_distress_signal(&sensors);
    println!("x={x}, y={y}");
    let solution = tuning_frequency(x, y);
    println!("solution: {solution}");
}

fn find_distress_signal(sensors: &[Sensor]) -> (i64, i64) {
    for y in 0..MAX_Y {
        let ruled_out_areas = exclusion_area(sensors, y, false);
        let possible_areas = ruled_out_areas.invert(0..MAX_X);
        //println!("Ruled out: {:?}", ruled_out_areas);
        //println!(" Inverted: {:?}", possible_areas);

        if !possible_areas.is_empty() {
            let x = possible_areas.start();
            return (x, y);
        }
    }

    panic!("Did not find distress signal!");
}

/// Return the ranges for
fn exclusion_area(sensors: &[Sensor], y: i64, remove_existing_beacons: bool) -> Ranges {
    let mut ranges = Ranges::new();
    for sensor in sensors.iter() {
        let range = sensor.range_at(y);
        ranges.add(range);
    }

    if remove_existing_beacons {
        for sensor in sensors.iter() {
            if sensor.closest_beacon.y == y {
                ranges.remove_point(sensor.closest_beacon.x);
            }
        }
    }

    ranges
}

fn tuning_frequency(x: i64, y: i64) -> i64 {
    x * 4000000 + y
}

impl Sensor {
    #[inline]
    const fn manhattan_radius(&self) -> i64 {
        (self.location.x - self.closest_beacon.x).abs()
            + (self.location.y - self.closest_beacon.y).abs()
    }

    fn range_at(&self, y: i64) -> Range<i64> {
        let difference = (self.y() - y).abs();

        // The signal is not in range.
        if difference > self.manhattan_radius() {
            return 0..0;
        }

        let lower = self.x() - (self.manhattan_radius() - difference);
        let upper = self.x() + (self.manhattan_radius() - difference) + 1;

        lower..upper
    }

    #[inline(always)]
    const fn x(&self) -> i64 {
        self.location.x
    }

    #[inline(always)]
    const fn y(&self) -> i64 {
        self.location.y
    }
}

impl Ranges {
    fn new() -> Self {
        Ranges { ranges: Vec::new() }
    }

    fn add(&mut self, new_range: Range<i64>) {
        if new_range.is_empty() {
            return;
        }

        // No ranges? Then this is the entire range.
        if self.ranges.is_empty() {
            self.ranges.push(new_range);
            return;
        }

        let mut intersections = Vec::with_capacity(self.ranges.len());

        // We need to figure out where the range goes.
        for (i, existing_range) in self.ranges.iter().enumerate() {
            if existing_range.start <= new_range.start && existing_range.end >= new_range.end {
                // The new range is fully-contained. We're done!
                return;
            }

            //      new:    |-----------)
            // existing:       |----)
            if new_range.start <= existing_range.start && new_range.end >= existing_range.end {
                // The new range fully contains the existing range.
                intersections.push(i);
                continue;
            }

            // Check if we intersect from the left:
            //      new:      |--)
            // existing:         |----)
            if new_range.start < existing_range.start && new_range.end >= existing_range.start {
                intersections.push(i);
                continue;
            }

            // Check if we intersect from the rightL
            //      new:         |----)
            // existing:      |----)
            if new_range.end >= existing_range.start && new_range.start <= existing_range.end {
                intersections.push(i);
            }
        }

        if intersections.is_empty() {
            // no intersections: just add it in there.
            self.ranges.push(new_range);
        } else {
            // intersections: merge the existing ranges.
            let first = &self.ranges[*intersections.first().unwrap()];
            let last = &self.ranges[*intersections.iter().rev().next().unwrap()];

            let lower = cmp::min(new_range.start, first.start);
            let upper = cmp::max(new_range.end, last.end);

            let mega_range = lower..upper;

            // remove the obsolete indices FROM THE END
            for &obsolete_idx in intersections.iter().rev() {
                // we'll sort later anyway, so it's okay to swap/remove:
                self.ranges.swap_remove(obsolete_idx);
            }

            self.ranges.push(mega_range);
        }

        // Ranges must be sorted:
        self.ranges.sort_by_key(|r| r.start);
    }

    fn invert(&self, over: Range<i64>) -> Ranges {
        let overlapping_ranges: Vec<Range<i64>> = self
            .ranges
            .iter()
            .filter(|range| range.end >= over.start)
            .filter(|range| range.start < over.end)
            .cloned()
            .collect();

        let mut result = Ranges::new();

        if overlapping_ranges.is_empty() {
            // it's the entire range:
            result.add(over);
            return result;
        }

        let first = overlapping_ranges.first().unwrap();
        let last = overlapping_ranges.iter().rev().next().unwrap();

        // the range fully subsumes the other tnage
        if first.start <= over.start && first.end >= over.end {
            // empty range:
            return result;
        }

        result.add(over.start..first.start);
        for (a, b) in overlapping_ranges
            .iter()
            .zip(overlapping_ranges.iter().skip(1))
        {
            result.add(a.end..b.start);
        }
        result.add(last.end..over.end);

        result
    }

    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    fn start(&self) -> i64 {
        self.ranges.first().expect("got start of empty range").start
    }

    fn remove_point(&mut self, point: i64) {
        let Some(i) = self._which_range_contains(point) else {
            // No range contains point. Ignore it.
            return;
        };

        // Pull out that old range:
        let old_range = self.ranges.swap_remove(i);

        // Make two new ranges excluding the point.
        let first_half = old_range.start..point;
        let second_half = (point + 1)..old_range.end;

        self.ranges.push(first_half);
        self.ranges.push(second_half);

        // Maintain the sorted invariant:
        self.ranges.sort_by_key(|r| r.start);
    }

    fn contain(&self, point: i64) -> bool {
        self._which_range_contains(point).is_some()
    }

    fn _which_range_contains(&self, point: i64) -> Option<usize> {
        for (i, range) in self.ranges.iter().enumerate() {
            if range.contains(&point) {
                return Some(i);
            }
        }

        None
    }

    #[allow(unused)]
    fn coverage(&self) -> i64 {
        self.ranges
            .iter()
            .map(|part| part.end - part.start)
            .sum::<i64>()
    }
}
