# Space Invaders

A [Space Invaders](https://en.wikipedia.org/wiki/Space_Invaders) clone written in Rust and Bevy. Mainly for learning Rust and Bevy :)

## Goal
* [x] Create a player cannon that moves side to side.
* [x] Create a few different types of alien invaders.
* [x] Enemies will move together in a grid. They cross the screen horizontally before dropping vertically and reversing their direction.
* [x] Add the ability for the player ship to fire a laser beam that travels up the screen. In the original game, you need to wait until the projectile either hits something or exits the screen before being able to fire again.
* [x] Add bombs that the enemies drop. The player’s laser can destroy enemy bombs. 
* [x] Only enemies that have a clear line of sight to the player will drop bombs.
* [x] Make sure that the player’s laser beams will destroy invaders, and the invader bombs will destroy the player.
* [x] Add a mothership that will cross the screen periodically. Destroying it will result in bonus points.
* [x] Add a UI that tracks the player score and lives left. The player starts with three lives.
* [ ] Indicate to the player when they lost the game
* [ ] Let the player progress to the next level (invaders start one line lower) after they destroyed all the invaders.

## Stretch goal
* [ ] Add a start screen
* [ ] Add nice pixel graphics like in the original game.
* [ ] Add sound effects similar to the original game
* [ ] Animte the graphics like in the original game
* [ ] The original game had bunkers that alien bombs and player's laser would slowly destroy. Some console ports had bunkers that would be destroyed after a certain number of hits. Others omitted bunkers entirely, or made different bunkers for different levels. Feel free to add any type of bunker to the game.
* [ ] Have fun with particle effects! You aren’t restricted to the original hardware, so feel free to add as much game juice (particles, sounds, screen shake) as possible.
