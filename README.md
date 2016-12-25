# RGB
[![Build Status](https://travis-ci.org/NivenT/RGB.svg?branch=master)](https://travis-ci.org/NivenT/RGB)

RGB (Rust Game Boy) is a simple emulator for the original game boy and the color game boy.

<img src="https://github.com/NivenT/RGB/blob/master/screenshots/img0.png" alt="Screenshot" width="400" height="300"/>
<img src="https://github.com/NivenT/RGB/blob/master/screenshots/img1.png" alt="Screenshot" width="400" height="300"/>
<img src="https://github.com/NivenT/RGB/blob/master/screenshots/img2.png" alt="Screenshot" width="400" height="300"/>
<img src="https://github.com/NivenT/RGB/blob/master/screenshots/img3.png" alt="Screenshot" width="400" height="300"/>
<img src="https://github.com/NivenT/RGB/blob/master/screenshots/img4.png" alt="Screenshot" width="800" height="200"/>

##How to Build
Install [Rust](https://www.rust-lang.org/en-US/) and run the following commands in a terminal
````
git clone https://github.com/NivenT/RGB
cd RGB
cargo build
````

In order for it to build, you must have SDL2 installed. On Ubuntu, run
````
sudo apt-get install libsdl2-dev
````

##How to Use
Before running the program, make sure to setup the settings.ini file. This is where you supply a path to the game to be loaded, tell the emulator which keyboard keys map to which gameboy buttons, and specify what hex colors the emulator should use for graphics. You can also supply a path to a binary file containg the gameboy BIOS. Even if you do not have a copy of the gameboy's BIOS (you supply a path to a nonexistent file), the emulator will still run. **If you supply a CGB BIOS file, the emulator will run as a gameboy color, but if you supply a monochrome gameboy BIOS file, the emulator will run as a monochrome gameboy. If no BIOS file is supplied, it will decide which to run as depending on if the loaded game was made for monochrome of color gameboys.** RGB uses SDL2 for window management and input handling, so check [here](https://github.com/AngryLawyer/rust-sdl2/blob/master/sdl2-sys/src/keycode.rs) for the values of each key.

Once settings.ini has been set up, start the program by running the following command from the project's main directory
```
cargo run --release
```

Once the program starts, it behaves like a gameboy with the keys you specified as the buttons.

##Special Input
Certain keys are special, and the emulator has built in responses for when they are pressed.

* P - Toggles whether or not emulation is paused
* D - Toggles whether or not the emulator prints debug information
* F - Emulates a single CPU instruction if paused
* M - Prompts for a starting and ending memory address. Emulator then prints the values stored in memory between those addresses (inclusive on starting and exclusive on ending)
* Esc - Exits program
* 1..0 - Runs the emulator at normal (double, triple, ..., up to 10x) speed

##Emulation Progress
###CPU
- [X] Implemented all normal instructions
- [X] Implemented all CB instructions
- [X] Implemented interrupts

###GPU
- [X] Can display tiles
- [X] Can display sprites
- [X] Flips sprites
- [X] CGB tiles
- [X] CGB sprites

###Memory
- [X] 32KB ROMs without banking
- [X] MBC1 memory banking
- [X] MBC2 memory banking
- [X] MBC3 memory banking
- [X] MBC5 memory banking
- [X] Can save games

###Input
- [X] Accepts input

###Sound
- [ ] Produces sound

##Known Bugs/Issues
* Some game behave strangly for unknown reasons
  * Ex. Dr. Mario freezes on the screen after the title screen
* CGB support is still a work in progress
  * CGB BIOS will get stuck in an infinite loop
