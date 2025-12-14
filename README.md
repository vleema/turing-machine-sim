# Turing machine simulator

Simple Turing machine simulator for helping me doing some exercise in a computability discipline.

## Usage

Since the project is made in rust, you can use it like this:

```bash
echo "input" | cargo r --release -- <machine_description_file>
```

The program will read from stdin until EOF. The exit code will be 0 if the machine ends in a accepting state and 1 otherwise.

The machine description comes from a file in the following format:

```plain
<alphabet>
<blank_character>
<accepting_states>
<initial_state>
<transitions>
```

Where:

```plain
alphabet = λ | <symbol> | <symbol> <symbol>
blank_character = <symbol>
accepting_states = λ | <state> | <state> <state>
initial_state = <state>
transitions = λ | <current_state> <read_symbol> <next_state> <write_symbol> <direction>
```

The $\lambda$ is the empty string.

The symbols are utf-8 characters and the states are unsigned integers, direction could be either R (right) or L (left). A example file would be the following:

```plain
t e s n i
_
0
1
1 t 2 n R
2 e 3 i R
3 s 4 c R
4 t 0 e R

1 n 2 t R
2 i 3 e R
3 c 4 s R
4 e 0 t R
```

This is a machine for substituting "nice" for "test" and "test" for "nice".

## Contributing

Feel free to do some pull requests or something, would be nice to have:

- [ ] A comment string for the description file, e.g. "#".
- [ ] A report mechanism for line number with parsing error.
