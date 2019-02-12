// NOTE: ALL VALUES ARE 0-INDEXED AND ONLY ADJUSTED FOR DISPLAY PURPOSES

const fn modulus(a: i32, b: i32) -> i32
{
    (((a % b) + b) % b)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}

impl Month {
    fn num_days(&self, is_leap_year: bool) -> i32 {
        use Month::*;

        match self {
            January => 31,
            February if is_leap_year => 29,
            February => 28,
            March => 31,
            April => 30,
            May => 31,
            June => 30,
            July => 31,
            August => 31,
            September => 30,
            October => 31,
            November => 30,
            December => 31,
        }
    }

    fn advance()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl Date {
    pub fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0)
    }

    pub fn days_in_this_month(&self) -> i32 {
        if self.month == 1 && self.is_leap_year() {
            29
        } else {
            day_counts
        }
    }

    fn reconcile(&mut self) {
        let mut year_offset = self.month / 12;
        self.month = modulus(self.month, 12);
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Time {
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub nanos: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moment {
    pub date: Date,
    pub time: Time,
}

impl Moment {
    fn reconcile(&mut self) {

    }
}

fn main() {
    println!("Hello, world!");
}
