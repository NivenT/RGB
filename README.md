# RGB
[![Build Status](https://travis-ci.org/NivenT/RGB.svg?branch=master)](https://travis-ci.org/NivenT/RGB) <--- Ignore this right now. It actually does build and run. Trust me

RGB (Rust Game Boy) is a simple emulator for the original game boy and the color game boy.

<img src="https://github.com/NivenT/RGB/blob/master/screenshots/img0.png" alt="Screenshot" width="400" height="300"/><img src="https://github.com/NivenT/RGB/blob/master/screenshots/img1.png" alt="Screenshot" width="400" height="300"/>
<img src="https://github.com/NivenT/RGB/blob/master/screenshots/img2.png" alt="Screenshot" width="400" height="300"/><img src="https://github.com/NivenT/RGB/blob/master/screenshots/img3.png" alt="Screenshot" width="400" height="300"/>
<img src="https://github.com/NivenT/RGB/blob/master/screenshots/img4.png" alt="Screenshot" width="800" height="200"/>

## How to Build
Install [Rust](https://www.rust-lang.org/en-US/) and run the following commands in a terminal
````
git clone https://github.com/NivenT/RGB
cd RGB
cargo build --release
````

In order for it to build, you must have SDL2 installed. On Ubuntu, run
````
sudo apt-get install libsdl2-dev
````

## How to Use
Before running the program, make sure to setup the settings.ini file. This is where you supply a path to the game to be loaded, tell the emulator which keyboard keys map to which gameboy buttons, and specify what hex colors the emulator should use for graphics. You can also supply a path to a binary file containg the gameboy BIOS. Even if you do not have a copy of the gameboy's BIOS (you supply a path to a nonexistent file), the emulator will still run. **If you supply a CGB BIOS file, the emulator will run as a gameboy color, but if you supply a monochrome gameboy BIOS file, the emulator will run as a monochrome gameboy. If no BIOS file is supplied, it will decide which to run as depending on if the loaded game was made for monochrome of color gameboys.** RGB uses SDL2 for window management and input handling, so check [here](https://github.com/AngryLawyer/rust-sdl2/blob/master/sdl2-sys/src/keycode.rs) for the values of each key.

Once settings.ini has been set up, start the program by running the following command from the project's main directory
```
cargo run --release
```

Once the program starts, it behaves like a gameboy with the keys you specified as the buttons.

## Special Inputs
Certain keys are special, and the emulator has built in responses for when they are pressed.

* P - Toggles whether or not emulation is paused
* D - Toggles whether or not the emulator displays debug information
  * R - Toggles whether or not register values should be included in the debug information
  * Up/Down - When paused, scrolls through the displayed debug information
* F - Emulates a single CPU instruction if paused
* M - Prompts for a starting and ending memory address. Emulator then prints the values stored in memory between those addresses (inclusive on starting and exclusive on ending)
* Esc - Exits program
* 1..0 - Runs the emulator at normal (double, triple, ..., up to 10x) speed

## Debugging
For helping with development, I've built some debugging features into the emulator.

* The main one is what happens when you press D. The emulator displays the opcodes its executing (along with their inputs) in real time, and you can scroll through this output via up/down arrows if emulation is paused.

<p align="center">
  <img src="https://github.com/NivenT/RGB/blob/master/screenshots/img5.png" alt="Screenshot" width="600" height="400"/>
</p>

* Another source of debugging information is the `disassembly.txt` file. Whenever a game is first loaded, the emulator (attempts to) disassemble its source code and print the results into this file for later viewing.

* Finally, `settings.ini` contains a `bios_breakpoint` flag. When this is set to `true`, the emulator will automaticallyy pause once the BIOS has finished running. This makes it easier to step through a game from the moment it begins.

## Known Bugs/Issues
* Dr. Mario freezes after pressing enter on the title screen
* Gameboy Color games may have slight graphical bugs
* There is no sound
* Sometimes (often), the emulator stops displaying the game when debug mode is entered for some unknown reason
* The emulator seems to be unable to play certain CGB Games (Ones using an MBC5 type cartridge)

If you find any other problems, please open an issue.
