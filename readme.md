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
    <figcaption><b>Screenshot of the start screen.</b>></figcaption>
</figure>

While the game is running you make your choice by clicking somewhere onto the column you want to make your move in.
<figure>
    <img src="Images/RunningScreen.png" alt="Image of the screen while game is in progress" width="300" height="300">
    <figcaption><b>Screenshot of the game in progress.</b>b></figcaption>
</figure>







## Prerequisits
