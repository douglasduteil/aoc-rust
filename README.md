<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2022

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
<!--- advent_readme_stars table --->

## 2022 Results

|                      Day                       | Part 1 | Part 2 |
| :--------------------------------------------: | :----: | :----: |
|  [Day 1](https://adventofcode.com/2022/day/1)  |   ⭐   |   ⭐   |
|  [Day 2](https://adventofcode.com/2022/day/2)  |   ⭐   |   ⭐   |
|  [Day 3](https://adventofcode.com/2022/day/3)  |   ⭐   |   ⭐   |
|  [Day 4](https://adventofcode.com/2022/day/4)  |   ⭐   |   ⭐   |
|  [Day 5](https://adventofcode.com/2022/day/5)  |   ⭐   |   ⭐   |
|  [Day 6](https://adventofcode.com/2022/day/6)  |   ⭐   |   ⭐   |
|  [Day 7](https://adventofcode.com/2022/day/7)  |   ⭐   |   ⭐   |
|  [Day 8](https://adventofcode.com/2022/day/8)  |   ⭐   |   ⭐   |
|  [Day 9](https://adventofcode.com/2022/day/9)  |   ⭐   |   ⭐   |
| [Day 10](https://adventofcode.com/2022/day/10) |   ⭐   |   ⭐   |
| [Day 11](https://adventofcode.com/2022/day/11) |   ⭐   |   ⭐   |
| [Day 12](https://adventofcode.com/2022/day/12) |   ⭐   |   ⭐   |

---

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created workspace "2022/day_01"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/bin/scaffold.rs#L11-L41) has _unit tests_ referencing its _example_ file. Use these unit tests to develop and debug your solution against the example input. For some puzzles, it might be easier to forgo the example file and hardcode inputs into the tests.

When editing a solution, `rust-analyzer` will display buttons for running / debugging unit tests above the unit test blocks.

### Download input & description for a day

> **Note**  
> This command requires [installing the aoc-cli crate](#download-puzzle-inputs-via-aoc-cli).

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# Saving puzzle description to "src/puzzles/01.md"...
# Downloading input for day 1, 2022...
# Saving puzzle input to "src/inputs/01.txt"...
# Done!
# ---
# 🎄 Successfully wrote input to "src/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "src/puzzles/01.md".
```

To download inputs for previous years, append the `--year/-y` flag. _(example: `cargo download 1 --year 2020`)_

Puzzle descriptions are stored in `src/puzzles` as markdown files. Puzzle inputs are not checked into git. [Reasoning](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3).

### Run solutions for a day

```sh
# example: `cargo solve day_2022_01`
cargo solve <day>

# output:
#     Running `target/debug/01`
# 🎄 Part 1 🎄
#
# 6 (elapsed: 37.03µs)
#
# 🎄 Part 2 🎄
#
# 9 (elapsed: 33.18µs)
```

`solve` is an alias for `cargo run --bin`. To run an optimized version for benchmarking, append the `--release` flag.

Displayed _timings_ show the raw execution time of your solution without overhead (e.g. file reads).

### Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# 🎄 Part 1 🎄
#
# 0 (elapsed: 170.00µs)
#
# 🎄 Part 2 🎄
#
# 0 (elapsed: 30.00µs)
# <...other days...>
# Total: 0.20ms
```

`all` is an alias for `cargo run`. To run an optimized version for benchmarking, use the `--release` flag.

_Total timing_ is computed from individual solution _timings_ and excludes as much overhead as possible.

### Run all solutions against the example input

```sh
cargo test
cargo watch -C 2022/day_01 -x test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### Format code

```sh
cargo fmt
```

### Lint code

```sh
cargo clippy
```

### Read puzzle description in terminal

> **Note**  
> This command requires [installing the aoc-cli crate](#download-puzzle-inputs-via-aoc-cli).

```sh
# example: `cargo read 1`
cargo read <day>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

To read inputs for previous years, append the `--year/-y` flag. _(example: `cargo read 1 --year 2020`)_

## Optional template features

### Download puzzle inputs via aoc-cli

1. Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) via cargo: `cargo install aoc-cli --version 0.7.0`
2. Create an `.adventofcode.session` file in your home directory and paste your session cookie[^1] into it. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie value.

Once installed, you can use the [download command](#download-input-for-a-day).

### Check code formatting in CI

Uncomment the `format` job in the `ci.yml` workflow to enable fmt checks in CI.

### Enable clippy lints in CI

Uncomment the `clippy` job in the `ci.yml` workflow to enable clippy checks in CI.

### Automatically track ⭐️ progress in the readme

This template includes [a Github action](https://github.com/k2bd/advent-readme-stars) that automatically updates the readme with your advent of code progress.

To enable it, complete the following steps:

#### 1. Create a private leaderboard

Go to the leaderboard page of the year you want to track and click _Private Leaderboard_. If you have not created a leaderboard yet, create one by clicking _Create It_. Your leaderboard should be accessible under `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.

#### 2. Set repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

-   `AOC_ENABLED`: This variable controls whether the workflow is enabled. Set it to `true` to enable the progress tracker.
-   `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option. Example: `3031`
-   `AOC_YEAR`: the year you want to track. Example: `2021`
-   `AOC_SESSION`: an active session[^2] for the advent of code website. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie.

✨ You can now run this action manually via the _Run workflow_ button on the workflow page. If you want the workflow to run automatically, uncomment the `schedule` section in the `readme-stars.yml` workflow file or add a `push` trigger.

### Use VS Code to debug your code

1.  Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
2.  Set breakpoints in your code. [^3]
3.  Click _Debug_ next to the unit test or the _main_ function. [^4]
4.  The debugger will halt your program at the specific line and allow you to inspect the local stack. [^5]

## Useful crates

-   [itertools](https://crates.io/crates/itertools): Extends iterators with extra methods and adaptors. Frequently useful for aoc puzzles.
-   [regex](https://crates.io/crates/regex): Official regular expressions implementation for Rust.

A curated list of popular crates can be found on [blessred.rs](https://blessed.rs/crates).

Do you have aoc-specific crate recommendations? [Share them!](https://github.com/fspoettel/advent-of-code-rust/edit/main/README.md)

## Common pitfalls

-   **Integer overflows:** This template uses 32-bit integers by default because it is generally faster - for example when packed in large arrays or structs - than using 64-bit integers everywhere. For some problems, solutions for real input might exceed 32-bit integer space. While this is checked and panics in `debug` mode, integers [wrap](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) in `release` mode, leading to wrong output when running your solution.

## Inspiring solutions

-   https://github.com/hxhelm/aoc2022 (day 7+)
-   https://github.com/SinshiSmith/aoc (day 15+)
-   https://github.com/nmhanson/aoc2022 (day 8+)
-   https://github.com/fspoettel/advent-of-code-2022 (day 20+)
-   https://github.com/tguichaoua/advent-of-code-2022-rust (day 18+)
-   https://github.com/rumpl/aoc2022 (day 3+)
-   https://github.com/tometo-dev/advent-of-code (day 11+)
-   https://github.com/peteanning/advent-of-code/blob/main/2022/rust (day 7+)

## Footnotes

[^1]: The session cookie might expire after a while (~1 month) which causes the downloads to fail. To fix this issue, refresh the `.adventofcode.session` file.
[^2]: The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
[^3]:
    <img src="https://user-images.githubusercontent.com/1682504/198838369-453dc22c-c645-4803-afe0-fc50d5a3f00c.png" alt="Set a breakpoint" width="450" />

[^4]:
    <img alt="Run debugger" src="https://user-images.githubusercontent.com/1682504/198838372-c89369f6-0d05-462e-a4c7-8cd97b0912e6.png" width="450" />

[^5]:
    <img alt="Inspect debugger state" src="https://user-images.githubusercontent.com/1682504/198838373-36df6996-23bf-4757-9335-0bc4c1db0276.png" width="450" />
