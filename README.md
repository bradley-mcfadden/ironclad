# Ironclad implementation

This is a command-line implementation of the game Ironclad by Frank Lantz,
which was commisioned for the book "Rules of Play" by Salen & Zimmerman.

## Rules

See [https://static1.squarespace.com/static/5b3eb1d15ffd20967f12cded/t/5b76120b4ae237e0a2fe5556/1534464525354/Ironclad.pdf](https://static1.squarespace.com/static/5b3eb1d15ffd20967f12cded/t/5b76120b4ae237e0a2fe5556/1534464525354/Ironclad.pdf).

## Playing the game

Run the following command to build the release executable and start it:

```
cargo build --release
```

The program will prompt the user for their intent -- what they want to do, and where 
they want to do it. The game proceeds until one player wins.

## Purpose

I wrote this program because I was intrigued by the interesting concept of
Ironclad as a board game, and also because I was interested in starting a 
small project to learn Rust. I'm glad to say that I did learn a lot about
working with Rust, like the aggressive compiler workflow, but I also was 
exposed to the standard library a lot, and I grew to really enjoy is iterator
system, as well as the automatic testing that Cargo provides.

I'd really like to do another project in Rust at some point. Once you get past
some of the pain points like only being able to have a single mutable reference,
not having dynamically sized types in structs without a Box, and incompatible
primitive integer types, it is really nice. I want to do more with dynamic
data structures next so I can learn about references more.


## Future work

Here are a few more additions that could be made for anyone interested:
- Implement a computer player.
- Add back navigation to the menus.
- Make a graphical interface for interacting with the game.
- Add the ability to surrender a losing game.
