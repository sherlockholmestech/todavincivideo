# todavincivideo

Converts video files to a format that can be imported via Davinci Resolve Studio for Linux. NOTICE: THIS IS FOR THE STUDIO VERSION, NOT THE FREE TO USE ONE!!!.

## Installation

### Prerequisites

#### FFmpeg

On Ubuntu/Debian:

```bash
sudo apt install ffmpeg
```
On Arch Linux:

```bash
sudo pacman -S ffmpeg
```
On openSUSE:

```bash
sudo zypper install ffmpeg
```
On Fedora:

```bash
sudo dnf install ffmpeg
```
On NixOS:

```bash
nix-env -i ffmpeg
```
Then, download the latest binary avaliable and extract it to a place in your PATH.

## Usage

```bash
todavincivideo input.mp4 output.mp4
```