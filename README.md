# Conway's Game of Life

## Intro

Conway's game of life is a cellular automation game designed by John Horton Conway in 1970. It takes no user input beyond a starting configuration. At this point, that's hard-coded. That's the next thing I have to work on.

## Rules

* Any live cell with fewer than two live neighbors dies from underpopulation
* Any live cell with two or three neighbors lives on
* Any live cell with more than three neighbors dies from overpopulation
* Any dead cell with three live neighbors is born again

## Run

You can run with

~~~
cargo run
~~~
