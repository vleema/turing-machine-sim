use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    process::ExitCode,
};

type State = usize;
type Symbol = char;
type Alphabet = HashSet<char>;
type Tape = VecDeque<char>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => Err("invalid character for direction")?,
        })
    }
}

#[derive(Debug)]
struct Machine {
    tape: Tape,
    head: usize,
    alphabet: Alphabet,
    blank: Symbol,
    accepting: HashSet<State>,
    init_state: State,
    state: State,
    transitions: HashMap<(State, Symbol), (State, Symbol, Direction)>,
}

impl Machine {
    fn new(
        alphabet: Alphabet,
        blank: Symbol,
        accepting: HashSet<State>,
        init_state: State,
        transitions: HashMap<(State, Symbol), (State, Symbol, Direction)>,
    ) -> Self {
        Self {
            tape: VecDeque::new(),
            head: 0,
            alphabet,
            blank,
            init_state,
            state: init_state,
            accepting,
            transitions,
        }
    }

    fn extend(&mut self, tape: &str) {
        self.tape.extend(tape.chars().map(|s| {
            if !self.alphabet.contains(&s) && s != self.blank {
                panic!("invalid tape symbol: '{s}'");
            } else {
                s
            }
        }));
    }

    fn describe(&self) {
        for (i, s) in self.tape.iter().enumerate() {
            if i == self.head {
                print!("({})", self.state);
            }
            print!("{s}");
        }
        println!()
    }

    fn execute(&mut self) -> bool {
        loop {
            self.describe();
            if !self.read() {
                break;
            }
        }
        self.accepting.contains(&self.state)
    }

    fn read(&mut self) -> bool {
        let Some((next, sym, dir)) = self.transitions.get(&(self.state, self.tape[self.head]))
        else {
            return false;
        };
        self.tape[self.head] = *sym;
        self.state = *next;
        self.head = match dir {
            Direction::Right => {
                if self.head >= usize::MAX - 1 {
                    panic!("end of tape");
                } else {
                    if self.head + 1 >= self.tape.len() {
                        self.tape.push_back(self.blank);
                    }
                    self.head + 1
                }
            }
            Direction::Left => {
                if self.head == 0 {
                    self.tape.push_front(self.blank);
                }
                self.head.min(self.head.wrapping_sub(1))
            }
        };
        true
    }

    fn tape(&self) -> String {
        self.tape.iter().collect::<String>()
    }

    fn reset(&mut self) {
        self.state = self.init_state;
        self.head = 0;
        self.tape.clear();
    }
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let Some(path) = std::env::args().nth(1) else {
        println!("usage: executable <machine_description_path>");
        return Err("please specify a path for them machine description".into());
    };

    let data = BufReader::new(File::open(path)?);
    let mut lines = data.lines();

    // TODO:
    // - Report line number with parsing error.
    // - Add comment lines preceded with # as comment string.
    let alphabet = lines
        .next()
        .ok_or("no alphabet")??
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect::<HashSet<Symbol>>();
    let blank = lines
        .next()
        .ok_or("no line for blank character")??
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .next()
        .ok_or("you should specify a blank character")?;
    let accepting = lines
        .next()
        .ok_or("no line for accepting states")??
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect::<HashSet<State>>();
    let init_state = lines
        .next()
        .ok_or("no line for initial state")??
        .split_whitespace()
        .flat_map(|s| s.parse::<State>().ok())
        .next()
        .ok_or("you should specify a intial state")?;
    let transitions: HashMap<_, _> = lines
        .map_while(Result::ok)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut iter = s.split_whitespace();
            let state = iter
                .next()
                .ok_or("the current state was not specified")?
                .parse::<State>()
                .map_err(|_| "invalid state")?;
            let head_sym = iter
                .next()
                .and_then(|s| s.chars().next())
                .ok_or("the head symbol was not specified")?;
            if !alphabet.contains(&head_sym) && head_sym != blank {
                return Err("invalid head symbol, doesn't exist in the alphabet");
            }
            let next_state = iter
                .next()
                .ok_or("the next state was not specified")?
                .parse::<State>()
                .map_err(|_| "invalid next state")?;
            let write_sym = iter
                .next()
                .and_then(|s| s.chars().next())
                .ok_or("the write symbol was not specified")?;
            if !alphabet.contains(&head_sym) && write_sym != blank {
                return Err("invalid write symbol, doesn't exist in the alphabet");
            }
            let dir = iter
                .next()
                .and_then(|s| s.chars().next())
                .ok_or("the direction was not specified")
                .and_then(Direction::try_from)?;
            Ok::<_, &str>(((state, head_sym), (next_state, write_sym, dir)))
        })
        .collect::<Result<_, _>>()?;

    let mut machine = Machine::new(alphabet, blank, accepting, init_state, transitions);

    let mut exit_code = 0;
    for line in std::io::stdin().lock().lines() {
        let tape = line?;
        machine.reset();
        machine.extend(&tape);
        exit_code = if machine.execute() { 0 } else { 1 };
        println!("{}", machine.tape());
    }

    Ok(exit_code.into())
}
