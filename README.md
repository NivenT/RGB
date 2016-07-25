# RGB
RGB (Rust Game Boy) is a simple emulator for the original game boy.

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

##How to Use
Before running the program, make sure to setup the settings.ini file. This is where you supply a path to the game to be loaded, tell the emulator which keyboard keys map to which gameboy buttons, and specify what hex colors the emulator should use for graphics. RGB uses SDL2 for window management and input handling, so check [here](https://github.com/AngryLawyer/rust-sdl2/blob/master/sdl2-sys/src/keycode.rs) for the values of each key.

Once settings.ini has been set up, start the program by running the following command from the project's main directory
```
cargo run
```

Once the program starts, it behaves like a gameboy with the keys you specified as the buttons.

##Special Input
Certain keys are special, and the emulator has built in responses for when they are pressed.

* P - Toggles whether or not emulation is paused
* D - Toggles whether or not the emulator prints debug information
* F - Emulates a single CPU instruction if paused
* M - Prompts for a starting and ending memory address. Emulator then prints the values stored in memory between those addresses (inclusive on starting and exclusive on ending)
* Esc - Exits program

##Emulation Progress
###CPU
- [X] Implemented all normal instructions
- [X] Implemented all CB instructions
- [X] Implemented interrupts

###GPU
- [X] Can display tiles
- [X] Can display sprites
- [ ] Flips sprites

###Memory
- [X] 32KB ROMs without banking
- [X] MBC1 memory  banking
- [ ] MBC2 memory  banking
- [ ] MBC3 memory  banking

###Input
- [X] Accepts input

###Sound
- [ ] Produces sound

##Known Bugs/Issues
* Dr. Mario freezes on the screen after the title screen
* Super Mario Land runs at 50% speed
* Time in Super Mario Land never goes below 460
