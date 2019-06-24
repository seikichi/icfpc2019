# 花脊山の家 (Hanase Yama no Ie)

- @cos65535
- @nojima
- @qwerty__
- @seikichi

Email: kondo.seikichi@gmail.com

## Requirements

- Rust: 1.35.0
- Cargo: 1.35.0

## How to Build (Wrapper)

```sh
> cargo build --release
```

## How to Run (Wrapper)

Beam Search AI!!!!!!!!!

```sh
> cargo run --bin main_beam --release < prob-XXX.desc > prob-XXX.sol
```

or

```sh
> cargo run --bin main_beam --release -- -b C < prob-XXX.desc > prob-XXX.sol # with initial boosters
```

## How to Build (Puzzle Solver)

```sh
> cargo build --release
```

## How to Run (Puzzle Solver)

```sh
> cargo run --bin puzzler --release < puzzle.cond > task.desc
```
