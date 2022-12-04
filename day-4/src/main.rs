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
}

#[derive(Copy, Clone, Debug, Inpt)]
#[inpt(regex = r",")]
struct ElfPair(#[inpt(before)] SectionRange, #[inpt(after)] SectionRange);

impl ElfPair {
    fn has_fully_contained_section(self) -> bool {
        let ElfPair(first, second) = self;
        first.fully_contains(second) || second.fully_contains(first)
    }
}

#[inpt::main]
fn main(pairs: Vec<ElfPair>) {
    let answer = pairs
        .into_iter()
        .filter(|pair| pair.has_fully_contained_section())
        .count();
    println!("{answer}");
}
