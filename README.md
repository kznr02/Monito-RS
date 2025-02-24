# Monito-RS: tool for control your monitor(s)

## Supported Features

- [x] adjust brightness
- [x] query brightness
- [x] enumerate displays
- [x] Adjust R G B

## Notice

!!! Your monitor should support DDC/CI protocol. If you want to adjust RGB, you should enable RGB customization feature manually. !!!

## Usage

- Pull the repo to your computer
- into the repo dir
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
