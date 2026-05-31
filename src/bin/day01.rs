use advent_of_code_2025::read_input;

enum Direction {
    Left,
    Right,
}

struct Node {
    prev: usize,
    next: usize,
}

struct Dial {
    nodes: [Node; 100],
    position: usize,
    password: usize,
}

impl Dial {
    fn new() -> Self {
        Dial {
            nodes: std::array::from_fn(|i| Node {
                prev: (i + 99) % 100,
                next: (i + 1) % 100,
            }),
            position: 50,
            password: 0,
        }
    }

    fn rotate(&mut self, direction: Direction, steps: usize) {
        for _ in 0..steps {
            self.position = match direction {
                Direction::Left => self.nodes[self.position].prev,
                Direction::Right => self.nodes[self.position].next,
            };
        }

        if self.position == 0 {
            self.password += 1;
        }
    }
}

fn main() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin/inputs/day01_input.txt");
    let content = read_input(&path);

    let mut dial = Dial::new();

    for rotation in content {
        let direction = match rotation.chars().next().unwrap() {
            'L' => Direction::Left,
            _ => Direction::Right,
        };
        let steps = rotation[1..].parse::<usize>().unwrap();

        dial.rotate(direction, steps);
    }

    println!("Password: {}", dial.password);
}
