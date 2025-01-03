# [Advent of Code 2024](https://adventofcode.com/2024)

## What is it?

<https://adventofcode.com/2024> in [`Rust`](https://www.rust-lang.org/).

I've decided to try Rust, but I won't finish reading its [book](https://doc.rust-lang.org/book/) before AoC2024 and I haven't had a chance to try anything except just reading that book.  
—This year, I'm not focusing on achieving faster times and might not complete most days. Last year, I was traveling, and this year, life has been a bit complicated and extremely busy, keeping me occupied seven days a week.

Since the opportunity has come, I have decided to dive into [`GoLang`](https://go.dev/) too. For some challenges, you might notice "This day is implemented in Go" message.

## How to execute

You can set up Rust yourself, but it’s easier to use a devcontainer.

Run the following command to execute the code:

```sh
cargo run {day} {…params}
```
or
```sh
go run . {{day}} {{…params}}
```

Replace `{day}` with the specific day's number (e.g., `1`) and `…params` will vary depending on the day. Simply run the command, and it will provide more details about the required parameters.

Verifying that all challenges produce the correct answers by running

```sh
cargo test
```
