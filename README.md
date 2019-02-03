# Hangman

A classic command-line word guessing game, written in Rust. You have to guess what the word is, one letter at a time. Guess wrong too many times, and you lose!

## Play

Requires Rust: https://rustup.rs

```sh
git clone https://github.com/ryanobeirne/hangman
cd hangman
cargo run --release
```

Gameplay is fairly simple: You're presented with a blank line. The number of blanks is the number of letters in the word. You'll be prompted to guess a letter. If the word contains the letter you enter, you'll fill in the blanks. If not, a part of the man is added to the gallows. Complete the entire man and you lose! Complete the entire word and you win!

## Disclaimer

This game contains about 47,000 random English words between 5 and 15 characters in length. I tried to remove as much profanity from the set as possible, but I cannot guarantee that it's 100% SFW. I can't even guarantee they're all real words. Good luck!