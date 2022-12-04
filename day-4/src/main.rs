use inpt::Inpt;

#[derive(Copy, Clone, Debug, Inpt)]
#[inpt(regex = r"(\d+)-(\d+)")]
struct SectionRange {
    start: i32,
    end: i32,
}

impl SectionRange {
    fn fully_contains(self, other: SectionRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(self, other: SectionRange) -> bool {
        if self.start < other.start {
            self.end >= other.start
        } else {
            other.end >= self.start
        }
    }
}

#[derive(Copy, Clone, Debug, Inpt)]
#[inpt(regex = r",")]
pub struct ElfPair(#[inpt(before)] SectionRange, #[inpt(after)] SectionRange);

impl ElfPair {
    pub fn has_fully_contained_section(self) -> bool {
        let ElfPair(first, second) = self;
        first.fully_contains(second) || second.fully_contains(first)
    }

    pub fn has_any_overlap(self) -> bool {
        let ElfPair(first, second) = self;
        first.overlaps(second)
    }
}

#[inpt::main]
fn main(pairs: Vec<ElfPair>) {
    let answer = pairs
        .into_iter()
        .filter(|pair| pair.has_any_overlap())
        .count();
    println!("{answer}");
}
