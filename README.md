# LOCO
Lines of Code CLI tool in Rust

## Description
LOCO - Lines Of COde was created as one of my first Rust Programs to learn the language an have some fun.
Basically it should detect common known Programming Languages and count the Lines of Code coded.

## Motivation
Since Python is/was my main programming language I was just eager to experience something new.
Something faster, something more safe (less writing of millions of testcases to somehow guarantee functionality), something statically types, something compiled, something where the ecosystem is less complex. 
I did dip my toes into some Go, but never created something.

## Usage
loco --language . => Retrieve Lines of Code metric for given Language in current folder. 

## Examples
loco --python .
loco --rust .
loco --cs .
