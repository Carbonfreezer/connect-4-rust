# Connect Four in Rust

## Overview
This project features a Negamax implementation of the game connect four with transposition tables and alpha beta pruning.
We do the calculations in a separate thread, so we can hide calculation time behind the animation of a falling stone.
The interface is drawn in pure OpenGL.

The project comes with a relative extensive documentation you can compile out the source code with 
```
cargo doc
```

## Using the Program
At the start of the program you get to choose, whether you want to play as first player (yellow) or second player
(blue). You do so, by simply clicking onto one of the circles.
<figure>
    <img src="Images/IntroScreen.png" alt="Image of the intro screen" width="300" height="300">
    <figcaption><b>Screenshot of the start screen.</b></figcaption>
</figure>

While the game is running you make your choice by clicking somewhere onto the column you want to make your move in.
<figure>
    <img src="Images/RunningScreen.png" alt="Image of the screen while game is in progress" width="300" height="300">
    <figcaption><b>Screenshot of the game in progress.</b></figcaption>
</figure>

Once the game is over, you the finish screen, where the relevant stones are highlighted and the winning or draw 
situation is also shown with the small icon above the screen. By clicking somewhere onto the screen the player goes
back to the start for game starter selection.

<figure>
    <img src="Images/GameOver.png" alt="Image of the screen for a game over situation." width="300" height="300">
    <figcaption><b>Screenshot of the game over state.</b></figcaption>
</figure>



# Getting Started with Rust
If you are new to Rust, here is a quick start:

1. Install Rust
2. Build and run the program.

## Install Rust
For *Linux* and *MacOS* users, open a terminal and enter the following command:
```
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```
For *Windows* users, get to the website
[Windows Installer](https://www.rust-lang.org/tools/install).

In both cases, you will wind up with mainly three programs:
- **rustup**: This is the installer and updater.
- **rustc**: This is the core compiler of the Rust language. You will rarely interface with it directly.
- **cargo**: This program contains the package manager (something like PiPy in Python) and a complete build system.
  This program is the central entry to the Rust world.

## Build, Run, and Test the various components
Once you have installed Rust, clone the directory from the repository, open a terminal, and navigate to the base directory
where the file *Cargo.toml* is contained. From here, you may now run several commands:

- **cargo doc --open**: Generates and opens the documentation in the browser.
- **cargo run -r** : Compiles in release mode and starts the app.


# License
The program is published under the MIT license as explained in the [license file](LICENSE).


