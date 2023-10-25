# Crusty JSON

A simple JSON parser CLI demo written in Rust. Probably the worst one out there but it was a lot of fun to make it.

It supports in-line input, file load, url fetching and stdin so you can pipe-in some stuff!

## Examples

- `crusty-json '{"name": "Fulano"}'`
- `crusty-json -f sample.json`
- `crusty-json -u https://jsonplaceholder.typicode.com/users`
- `cat sample.json | crusty-json`

## TL;DR

Looking to learn and experiment with different things, I ended up making a JSON parser. First in Python, then in C++ and finally in Rust (the best of the three).

In the Python version I was exploring the concepts, in the C++ version I was exploring the pain and finally in the Rust version I found God (just kidding). Rust allowed me to express the solution in the most elegant and intuitive way and it was the one that I ended up delving into and therefore the one that led me to create this repo.

_\- DevCorvus_
