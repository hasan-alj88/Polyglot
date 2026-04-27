// Conditional Overlap Detection Algorithm

pub enum BranchType {
    IntOrFloat,
    EnumOrBool,
    StringOrFlexible,
    Compound,
}

pub struct Interval {
    pub lo: f64,
    pub hi: f64,
    pub lo_inclusive: bool,
    pub hi_inclusive: bool,
}

impl Interval {
    pub fn intersects(&self, other: &Interval) -> bool {
        let intersection_lo = f64::max(self.lo, other.lo);
        let intersection_hi = f64::min(self.hi, other.hi);
        
        if intersection_lo < intersection_hi {
            true
        } else if (intersection_lo - intersection_hi).abs() < f64::EPSILON {
            // Check boundary inclusiveness
            let self_has_point = (self.lo == intersection_lo && self.lo_inclusive) || (self.hi == intersection_hi && self.hi_inclusive);
            let other_has_point = (other.lo == intersection_lo && other.lo_inclusive) || (other.hi == intersection_hi && other.hi_inclusive);
            self_has_point && other_has_point
        } else {
            false
        }
    }
}

pub fn check_overlap_numeric(intervals: &[(usize, Interval)]) -> Option<(usize, usize)> {
    for i in 0..intervals.len() {
        for j in (i + 1)..intervals.len() {
            if intervals[i].1.intersects(&intervals[j].1) {
                return Some((intervals[i].0, intervals[j].0));
            }
        }
    }
    None
}

pub fn check_overlap_string<'a>(literals: &[(usize, &'a str)]) -> Option<(usize, usize)> {
    for i in 0..literals.len() {
        for j in (i + 1)..literals.len() {
            if literals[i].1 == literals[j].1 {
                return Some((literals[i].0, literals[j].0));
            }
        }
    }
    None
}

// Bitset for Enum/Bool is a simple u64 mask for variants
pub fn check_overlap_enum(bitsets: &[(usize, u64)]) -> Option<(usize, usize)> {
    for i in 0..bitsets.len() {
        for j in (i + 1)..bitsets.len() {
            if (bitsets[i].1 & bitsets[j].1) != 0 {
                return Some((bitsets[i].0, bitsets[j].0));
            }
        }
    }
    None
}
