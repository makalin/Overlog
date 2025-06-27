# Fonts Directory

This directory contains fonts used by Overlog for rendering text overlays.

## Default Font

The application expects a `Roboto-Regular.ttf` file in this directory. You can download it from Google Fonts or use any other TTF font by renaming it.

## Adding Custom Fonts

1. Place your TTF font file in this directory
2. Update the font loading code in `src/renderer.rs` to use your font
3. Make sure the font supports the characters you need for your telemetry data

## Font Requirements

- Format: TTF (TrueType Font)
- Should support basic Latin characters and numbers
- Consider font weight and readability for overlay display

## Download Roboto Font

You can download the Roboto font from:
https://fonts.google.com/specimen/Roboto

Or use a system font by modifying the font loading code. 