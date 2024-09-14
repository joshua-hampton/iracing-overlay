# iracing-overlay

Very early, prototype, proof-of-concept, free, open source overlays for iRacing.

## Install

### Release

The quickest way to get started is to download and run the installer from from the latest release. This will install the executables into `C:\Program Files (x86)\iRacing Overlays` and create shortcuts on the Desktop and the Start Menu for the main application.

### Build from source

The project is written in Rust. After cloning the code, it can be build using `cargo build` as you would with any Rust project.

## Important notes

This is project is at a very early stage, and is mostly just a proof-of-concept at the moment.

* Only close the overlay apps using the toggle in the main app or by closing the main app itself, rather than using the close button on each window. The title bar for the overlay windows will be removed once window positioning has been sorted out.
* The overlays might not show at the moment if the iRacing simulator is not running, and will need toggling off and back on again to work.

## Plans

The initial plan for this code is to focus on firstly making the app and the overlays as customisable as possible, with regard to the general look and appearance. Once progress has been made with the appearance for the main app and the current overlays, then more information and overlays can be added.

## License

This repository, and the code within, is covered by the GPL-3.0 License.