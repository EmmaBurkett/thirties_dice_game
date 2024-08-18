# Are you here from my resume? Are you wondering WHERE THE CALANDER PROGRAM IS????
This is a Rust based dice game. The calender program cannot be posted online as it is a school project and out of respect for future above and beyond students I will not post my solution.



# Overview

My initial goal for this project was to understand rust as a language, but a side goal developed rather quickly. My goal was not only to learn rust but also to understand ownership and lifetime parameters. I've struggled with C++ for a long time, because I didn't understand concepts like these, but now if I try to write another program in C++ I would really enjoy it. Rust is an amazing language and Visual Studio Code is a great learning tool.

This is a 30's dice game, and the goal is to keep your 'main' points.
###### Rules

* Player one throws 6 six dice with the goal to have those dice total to, or over, 30 points.
* The player sets aside the dice they would like to keep, at least one die, and rolls again.
* They continue rolling until all six dice have been set aside. 
###### Attack

* If the dice total over thirty they get to attack the other player's main points.
* Take the dice total and subtract thirty: Ex. 35 - 30 = 5
* Then roll all six dice
* If the player rolled any fives they set those dice aside
* If no fives were rolled then the 'attack' stops
* If a five was rolled then roll the remaining dice do this until no fives are rolled or all dice have been rolled.
* Take the total of all the dice that rolled fives and subtract that from the other player's main score. 
###### No Attack

* If the dice total is not over thirty then take 30 and subtract the dice total Ex. (30 - 25 = 5)
* Subtract the answer (5) from your main points.
###### Repeat
* Repeat for each player until someone's main score is zero.

# My purpose in writing this software was to...

        Entertain my friends

# Whimsical 
             This was a rough draft of my function hierarchy
![whimsical](https://user-images.githubusercontent.com/77114845/143724111-6c8689f8-7046-48a1-b84e-6bcef7893ea3.PNG)


## [Software Demo Video](https://youtu.be/FmDq_GhdF9g)

# Development Environment

* Rust
* Visual Studio Code
* Windows PowerShell

# Packages
* Colored 2.0.0 -> Colors terminal
* Ansi_term 0.12.1 -> Colors terminal
* Rand 0.8.4 -> Allows for randomization

# Libraries 
* rand::Rng; -> Rng trait defines methods that random number generators implement 
* std::io -> input output library from the std or standard library 
* colored::Colorize -> Colors terminal
* ansi_term::Style -> Colors terminal

# Useful Websites

* [rust-lang - vector.pop](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4547762e00f785f3abcfdd8b44a5d077)
* [programming-idioms - vector.pop](https://programming-idioms.org/idiom/226/delete-last-element-from-list/4074/rust)
* [Grepper - i32 as usize](https://www.codegrepper.com/code-examples/rust/convert+i32+to+usize+rust)
* [Stackoverflow - slice](https://stackoverflow.com/questions/39785597/how-do-i-get-a-slice-of-a-vect-in-rust)
* [joshmcguigan - Initialize an empty vector](https://www.joshmcguigan.com/blog/array-initialization-rust/)
* [stackoverflow - dereference](https://stackoverflow.com/questions/23920968/why-does-the-binary-operator-not-work-with-two-mut-int)
* [rust-lang - tuples](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)
* [Whimsical](https://whimsical.com/rust-30s-game-UPAFCWM4Kqu7D7n9Q6DkZq)
* [tutorialspoint - slices](https://www.tutorialspoint.com/rust/rust_slices.htm)
* [rust-lang - x++ vs x += 1](https://users.rust-lang.org/t/why-cant-i-increment-a-variable-like-this/18287/2)
* [docs.rs - Ansi_term](https://docs.rs/crate/ansi_term/0.6.3)
* [stackoverflow - if x in vector](https://stackoverflow.com/questions/58368801/how-do-i-check-if-a-thing-is-in-a-vector)


# Future Work
* A program which interacts with HTML and sets up a schedule for the day

# Fix and Add
* Different user interface than terminal
* A brief description of the rules at the beginning of the game
* Creating a new structure for the structs - i.e. having a player struct
* More colors
