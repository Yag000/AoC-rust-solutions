use aoc_star::star;

enum Rotation {
    Left(i32),
    Right(i32),
}

impl From<String> for Rotation {
    fn from(value: String) -> Self {
        let mut chars = value.chars();

        let dir = chars.next().unwrap();

        let num: i32 = chars.as_str().parse().unwrap();

        match dir {
            'L' => Rotation::Left(num),
            'R' => Rotation::Right(num),
            _ => panic!(),
        }
    }
}

pub struct Counter(i32);

impl Counter {
    fn update(self, r: Rotation) -> (Self, i32) {
        let Counter(pos) = self;
        match r {
            Rotation::Left(n) => {
                if pos - n >= 0 {
                    (Counter(pos - n), if pos - n == 0 { 1 } else { 0 })
                } else {
                    let new_pos = (pos - n).rem_euclid(100);
                    let times = ((pos - n).abs()) / 100 + (if pos == 0 { 0 } else { 1 });
                    (Counter(new_pos), times)
                }
            }
            Rotation::Right(n) => {
                let new_pos = (pos + n) % 100;
                let times = (pos + n) / 100;
                (Counter(new_pos), times)
            }
        }
    }
}

#[star(day = 1, part = 1)]
fn solve_part1(input: String) -> String {
    let mut counter = Counter(50);
    let mut value = 0;
    for line in input.lines() {
        let (new_counter, _) = counter.update(line.to_string().into());
        counter = new_counter;
        if let Counter(0) = counter {
            value += 1;
        }
    }

    return value.to_string();
}

#[star(day = 1, part = 2)]
fn solve_part2(input: String) -> String {
    let mut counter = Counter(50);
    let mut value = 0;
    for line in input.lines() {
        let (new_counter, size) = counter.update(line.to_string().into());
        counter = new_counter;
        value += size;
    }

    return value.to_string();
}
