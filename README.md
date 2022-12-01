# This repo contains the solution to the EI-SS 2022/2023 XOR challenge.

The file `ciphers.txt` contains the given ciphers.
The file `guesses.txt` contains the output from every guess, from start to finish.

## Usage:
Run and input your guess when prompted, guess ends on newline. 

Every guess appends to the `guesses.txt` file, with the addition of a timestamp on top like this example: `Wed Nov 30 23:53:32 2022`, followed by your guess.

The program will always try to write the XORed value converted to UTF8, but, when it isn't possible, it replaces it with the `�` character.
