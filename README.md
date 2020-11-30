# `v5prob`
A simple utility for calculating the probabilities of various outcomes in the *Vampire: The Masquerade 5th Edition* (V5) ruleset. By supplying a dice pool, hunger pool, and difficulty, the tool will simulate rolls to give the approximate probability of receiving a critical, messy critical, success, failure, total failure, or bestial failure.

## Usage
`./v5prob <pool> <hunger> <difficulty>`

## Build
With Rust and Cargo installed, simply run `cargo run` with the appropriate arguments (above). Running the release build is much faster than debug.
