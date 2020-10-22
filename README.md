# o64 - 64 Operator Random Drone FM Synth

This is a simple FM synthensizer that uses 64 operators to randomly generate a drone. By default, the current time is used as a seed for the random number generator but this can also be set as a command line argument to replay the same drone.

The operators are chained together in either 2, 4 or 8 groups and each operator has its own envelope. Every operator, envelope and filter value is mutated over time resulting in an ever-changing sound.

## Run

```bash
cargo run --release
```
