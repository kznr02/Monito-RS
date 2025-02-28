# Monito-RS: tool for control your monitor(s)

## Supported Features

- [x] adjust brightness
- [x] query brightness
- [x] enumerate displays
- [x] Adjust R G B

## Notice

!!! Your monitor should support DDC/CI protocol. If you want to adjust RGB, you should enable RGB customization feature manually. !!!

## CLI Usage

### List All Presented Monitors

`monito-rs list`

### Get Monitor Parameter

`monito-rs get {brightness|color}` Fetch all monitors

`monito-rs get -m {id} {brightness|color}` Fetch Monitor{id}

### Set Monitor Parameter

`monitor-rs set {brightness|color} {value}` Set all monitors

`monito-rs set -m {id} {brightness|color} {value}` Set monitor{id}

## CLI Build Instruction

- Pull the repo to your computer
- into the repo dir
- excute `cargo run --package monito-cli`
- read the help instruction to use

## GUI Build Instruction

- Pull the repo to your computer
- into the repo/monito-gui dir
- excute `deno install`
- Finally run `deno task tauri dev`

## Minimum dev environment requirement

- deno v2
- MSRV(Minimum supported rust version) 1.85+

## How to contribute

Seriously I don't know, if you find any bug or any suggestion, pls tell me or PR me : )

## RoadMap

- [ ] Beautiful Readme
- [ ] Tray Bar quick widget support
- [ ] Configuration file support
- [ ] Custom VCP command support
- [ ] Switching configurations over time
- [ ] More smooth and beautiful UI
