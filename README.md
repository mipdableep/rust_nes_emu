# Our Awsome NES Emulator

<!-- TOC -->

* [Our Awsome NES Emulator](#our-awsome-nes-emulator)
    * [Current Progress - ~85%](#current-progress---85)
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
        * [The Emulator](#the-emulator)
            * [Change the game](#change-the-game)
        * [Snake game](#snake-game-1)

<!-- TOC -->

### Current Progress - ~85%

Just finished the cpu, and ROM loading

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
8. Sources about the graphical process:
    1. [explanation about the background rendering process](https://www.nesdev.org/wiki/PPU_rendering)
    2. [explanation about sprite evaluation](https://www.nesdev.org/wiki/PPU_sprite_evaluation)
    3. [picture that summarizes the rendering process](https://www.nesdev.org/wiki/File:Ppu.svg)

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

## Full Emulator

After that, we just needed to implement the ppu (Picture Processing Unit), which is the part that is responsible for
actually drawing the picture. On one leg, this part has its own memory, and the cpu can communicate with the ppu memory
via special registers, that are mapped to certain parts of the memory. Once the ppu memory is full, it draws the screen
from this memory. This sounds easy, and shouldn't take much time, right?

... So we had to refactor major parts of our code, to allow both the cpu and the ppu to read from the same bus (due to a
design choice we made, that said the ppu and the cpu should both have a reference to the bus, instead of the more
standard approach of having the ppu reference in the bus). This, combined with the quirks of how the nes is rendering
its screen, made us change the memory read function to be change the memory (get `mut &self` instead of `&self`), which
in part caused a very weird bug that caused the background tiles to "skip" about every tile (the fix was either in
commit
`95bb781fd0083c01fede4296d84dd3d910167aff` or in commit `ec2db0cfacc68b887e072c551ecc71ac35b47822`). Rust tried to warn
us about this change in mutability, but we ignored and
paid the price.

So without further ado, let's see some images and gifs from the developing process!

### Cool images

We started by trying to emulate packman. We started by drawing the background tiles, with pre-determined palette.
![pacman_sprites_wrong_colors.jpeg](readme_images%2Fpacman_sprites_wrong_colors.jpeg)

Next, we got this wierd image instead of the home screen. That was, until we noticed the blinks were just in the place
the home screen blinked (when showing all the characters, and afterward when the big orbs are in the game). So we
realized we used the wrong bank for background sprites, and quickly fixed it.
![pacman_incorrect_sprites.gif](readme_images%2Fpacman_incorrect_sprites.gif)

And now for a quick build montage:

<figure>
  <img
  src="readme_images%2Fpacman_home_screen_with_extra_spaces.jpeg"
  alt="PacMan home screen with extra spaces ">
  <figcaption><p style="text-align: center;">The pacman loading screen with all the extra spaces (before we fixed the memory reading). The color palette is also wrong</p></figcaption>
</figure>




<figure>
<p float="left">
  <img src="readme_images%2Fpacman_homescreen_about_colors.jpeg"
  alt="PacMan home screen wrong colors" width="31%" />
  <img src="readme_images%2Fpacman_homescreen_wrong_colors.jpeg"
  alt="PacMan home screen with wrong color palette" width="31%" /> 
  <img src="readme_images%2Fpacman_good_homescreen.jpeg"
  alt="PacMan good home screen" width="31%" />
</p>
  <figcaption><p style="text-align: center;">We fixed the spaces, and gradually fixed the colors. In the first picture we still used a pre-determined palette. Then we  read from the game palette, but mixed the nametable high byte with the nametable low byte. Surely this won't happen again with the sprites, right?</p></figcaption>
</figure>


<figure>
<p float="left">
  <img
  src="readme_images%2Fpacman_good_homescreen.jpeg"
  alt="PacMan good home screen" width="31%">
  <img
  src="readme_images%2Fpacman_characters_intro_no_sprites.jpeg"
  alt="PacMan loading screen without sprites" width="31%">
  <img
  src="readme_images%2Fpacman_board_no_sprites.jpeg"
  alt="PacMan with just background, no sprites" width="31%">
</p>
  <figcaption><p style="text-align: center;">These three images show the game without sprites</p></figcaption>
</figure>


 <video width="320" height="240" controls>
  <source src="readme_images%2Fpacman_wierd_sprites.mp4" type="video/mp4">
</video> 
 After we started to tackle the sprites, we got this funny video. However, we quickly fixed it

<figure>
  <img
  src="readme_images%2Fpacman_board_with_sprites.jpeg"
  alt="PacMan with sprites">
  <figcaption><p style="text-align: center;">And now we added character sprites!</p></figcaption>
</figure>

 <video width="320" height="240" controls>
  <source src="readme_images%2Fpacman_good.mp4" type="video/mp4">
</video> 

A glorious pacman gameplay!


 <video width="320" height="240" controls>
  <source src="readme_images%2Fmario_gameplay_no_scrolling.mp4" type="video/mp4">
</video> 

Now we can play mario, without the scrolling part! In the future, we implemented the screen scrolling.

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

We check only upto line 5004, which is pc 0xc6bd, since there the opcode is 0x04, which is undocumented opcode (read
about it!).

To generate our logs run

```bash
cargo run --package nes_emulator --bin gen_cpu_tests_logs -- ./full_tests/nestest.nes ./full_tests/foo.txt 0xc6bd
```

To run the python script, that both generate the logs (using a subprocess of the previous command) and compare them (
using itself), run

```bash
python3 .\full_tests\compare_logs.py
```

You may need to install `pydantic` before running the script (`pip install pydantic`).

### The Emulator

Using cargo you can do

```bash
cargo run --package nes_emulator --bin nes_main
```

Or in release mode

```bash
cargo run --package nes_emulator --bin nes_main --release
```

#### Change the game

Edit the `nes_main/main.rs` file.

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
