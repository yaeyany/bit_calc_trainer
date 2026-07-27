# To-Do List

A command-line Bit-Decimal-Bit application written in Rust as part of my rust learning journey.

## Features

- Test your conversion from bits to decimal and vice-versa
- Simple command-line interface

## Binary to Decimal

Each bit in a binary number represents a power of two, starting from the rightmost bit (2⁰). Moving left, each position doubles in value: 1, 2, 4, 8, 16, 32, and so on. To convert a binary number to decimal, add together the values of every position containing a `1`.

Example:

1011

= 1×8 + 0×4 + 1×2 + 1×1

= 8 + 2 + 1

= 11

## Decimal to Binary

To convert a decimal number to binary, repeatedly divide the number by 2 and record the remainder after each division. Continue until the quotient becomes 0. The binary number is formed by reading the remainders from bottom to top.

Example:

13

13 ÷ 2 = 6, remainder 1

6 ÷ 2 = 3, remainder 0

3 ÷ 2 = 1, remainder 1

1 ÷ 2 = 0, remainder 1

Reading the remainders from bottom to top:

1101

## Technologies

- Rust

## Running

```bash
cargo run
```

## Future Improvements

- Not planned, works as it is