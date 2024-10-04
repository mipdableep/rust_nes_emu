# Our Awsome NES Emulator

<!-- TOC -->

* [Our Awsome NES Emulator](#our-awsome-nes-emulator)
    * [Current Progress - ~25%](#current-progress---25)
    * [Background](#background)
        * [Why?](#why)
        * [How do we Code?](#how-do-we-code)
    * [Technical](#technical)
    * [Snake Game!](#snake-game)
        * [Our Snake Game](#our-snake-game)
    * [How to run...](#how-to-run)
        * [Tests](#tests)
            * [Unittests](#unittests)
            * [Full CPU Tests](#full-cpu-tests)
        * [Snake game](#snake-game-1)

<!-- TOC -->

### Current Progress - ~25%

Just finished the cpu

## Background

The NES (Nintendo Entertainment System)  is a fantastic console able to run awsome games such as the original super
mario brothers, tetris, pacman and even super mario bros 3!

Rust is relatively low-level language, focused on memory safety and developer clarity, without sacrificing
high-performance. This makes it a good language to emulate the modified 6502 chip which is the core of the nes without
worrying too much about optimization even in relatively "bad" computers.

### Why?

This project has two main purposes: to help us learn rust, and to let us code with the homie.

### How do we Code?

This repo also tries to help us maintain "good" habits. So hopefully everything will be tested (at least in the CPU,
which have a lot of tricky pars), and pre-push hooks will make sure our code is formatted and passes all the tests
before we push it (it also make sure we don't have any uncommited changes).

## Technical

We follow (not strictly) the guide in [here](https://bugzmanov.github.io/nes_ebook/).
Some other useful links are:

1. [the rust guidebook](https://doc.rust-lang.org/stable/book/)
2. [a guide with details about 6502 commands](https://www.nesdev.org/obelisk-6502-guide/reference.html)
3. [another guide with even more details about 6502 commands](http://www.6502.org/tutorials/6502opcodes.html)
4. [another guide with more human friendly explanation, but with less details](https://www.pagetable.com/c64ref/6502/?tab=2#)
5. [someones repo about nes emulator in c](https://github.com/ObaraEmmanuel/NES/tree/master)
6. [another new emulator repo, in python](https://github.com/jameskmurphy/nes/tree/main)
7. [interactive 6502 tutorial, with the added benefit of being able to see what results some opcodes should give in given scenarios](https://skilldrick.github.io/easy6502/)

## Snake Game!

After we implemented the cpu, we created custom screen to implement the snake game
found [here](https://gist.github.com/wkjagt/9043907). This is a different binary named "SNAKE" (more about it in the how
to run section). Before you run the snake game, make sure you installed sdl2
per [these](https://github.com/Rust-SDL2/rust-sdl2) instructions. You may need to include the dll near where you have
your exe, depending on your os.

### Our Snake Game

You play the snake game with the wasd keys. The "P" key is used to pause the game at any point. The game is also
auto-paused after death (and can be released by pressing the "P" key again, or by waiting 10 seconds).
![snake_game.gif](readme_images%2Fsnake_game.gif)

## How to run...

### Tests

#### Unittests

Unittests are written in rust and can be run using

```bash
cargo test
```

#### Full CPU Tests

We also have tests on the full CPU based on a known test suite for nes
named [nestest](https://github.com/dbousamra/hnes/tree/master/roms/tests/cpu). The test contains a ROM (can be found in
`full_tests/nestes.nes`), and the results of the ROM (`full_tests/nestes_result_good.log`). Our tests have two parts -
first we emulate the Running of the `nestes.nes` using our cpu, and write the result after each opcode to `.txt` file in
`full_tests` (this is in `.gitignore`). Then, we compare our result to the good result using a python script in the same
directory.

We check only upto line 5004, since there the opcode is 0x04, which is undocumented opcode (read about it!).

To generate our logs run

```bash
cargo run --package nes_emulator --bin gen_cpu_tests_logs -- ./full_tests/nestest.nes ./full_tests/foo.txt
```

To run the python script, that both generate the logs (using a subprocess of the previous command) and compare them (
using itself), run

```bash
python3 .\full_tests\compare_logs.py
```

You may need to install `pydantic` before running the script (`pip install pydantic`).

### Snake game

```bash
cargo run --bin SNAKE
```

There is even (kind of) cli! you can choose to load the snake game from a dump (.nes file found in snake_game directory)
or "hard coded" (the values are hard coded in the code). You can also choose to see some kind of basic debug print (that
prints the current pc, opcode, and two bytes after the opcode). Run it with

```bash
cargo run --bin SNAKE hard_coded/dump true/false
```

The values also have default values - dump and false (no debug print), but you must set the hard_coded/dump argument
before passing the debug argument.
