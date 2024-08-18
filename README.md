# iracing-overlay

Very early, prototype, proof-of-concept, free, open source overlays for iRacing.

## Install

### Release

The quickest way to get started is to download the executable from from the latest release, and run the executable after launching an iRacing session.

### Build from source

The project is written in Rust. After cloning the code, it can be build using `cargo build` as you would with any Rust project.

## Important notes

This is project is at a very early stage, and is mostly just a proof-of-concept at the moment. As such, currently the executable will quietly and immediately exit if you do not have the iRacing simulator running, and the window will not automatically stay on top of iRacing.