# cli-videoeditor
## What is this project
This project is developed for video editing useing CLI.
I recommend this to craftsman of FFmpeg like me.

## How to use
### Basic Use
This has mode.
First, you have to select menu what you want.
### Concat
This mode is `ffmpeg -f concat` command.
This will make list.txt file.
And execute `ffmoeg -f concat -i list.txt <output file name>`.


# Dev memo
I decide to use clap-ts/Clap, but wait...
