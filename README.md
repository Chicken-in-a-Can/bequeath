# BEQUEATH

Game to try out making a magic system  
Making the engine in Rust, and the language  
Game being made in the language I'm writing, and partially in Rust too  

## Project structure
 - `bengine`: Game engine (Bequeath engine)
 - `beqlang`: Game language (Bequeath language)
 - `beqlang/syntax.guide`: Syntax guide for Beqeath programming language (Beqlang)
 - `bequeath`: Bequeath
 - `bequeath/assets`: Music, fonts, sprites, sfx, and more

## Libraries used:
 - Rust standard libraries
 - `tiny-skia`
 - `softbuffer`
 - `winit`
 - `image`
 - `rodio`

## Checklist for self-validation when I complete something
### Engine
 - [x] Create windows
 - [x] Render sprites
 - [x] Move sprites
 - [ ] Play audio
 - [x] Accept keyboard inputs
 - [x] Run  at stable framerate
 - [x] Multithread everything

### Language
 - [x] Accept command line arguments
 - [x] Read in imports
 - [ ] Get variables functioning
 - [ ] Get functions functioning
 - [ ] Get objects functioning
 - [ ] Integrate with engine
 - [ ] Multithread everything to compensate for the inefficiency caused by my reliance on OOP

### Game
 - [ ] Freeform magic circle creation
 - [ ] Roguelike
 - [ ] Procedurally generated spire for each attempt
