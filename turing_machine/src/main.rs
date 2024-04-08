#[derive(PartialEq)]
enum State {
    Start,
    FindOne,
    Halt,
}

enum Direction {
    Left,
    Right,
}

struct TuringMachine {
    tape: Vec<u8>,
    head_position: usize,
    state: State,
}

impl TuringMachine {
    fn new(tape: Vec<u8>, head_position: usize) -> TuringMachine {
        TuringMachine {
            tape,
            head_position,
            state: State::Start,
        }
    }

    fn step(&mut self) {
        match self.state {
            State::Start => {
                self.state = State::FindOne;
            }
            State::FindOne => {
                if self.tape[self.head_position] == 1 {
                    self.tape[self.head_position] = 0;
                    self.state = State::Halt;
                } else {
                    self.head_position += 1; // move right
                }
            }
            State::Halt => {
                // Machine halts
                return;
            }
        }
    }

    fn run(&mut self) {
        while self.state != State::Halt {
            self.step();
        }
    }
}

fn main() {
    let tape = vec![0, 0, 1, 0, 1]; // Sample tape
    let mut tm = TuringMachine::new(tape, 0); // Start at position 0
    tm.run();
    println!("Final tape: {:?}", tm.tape);
}