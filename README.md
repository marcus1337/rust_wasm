### Prerequisites

Create a folder named `assets` and put media resources used by the app there. Make sure to ship the `assets` folder along with the executable or wasm-binary.

This example uses fonts from Google, `Fira Sans`. (https://fonts.google.com/specimen/Fira+Sans).
Create a `fonts` folder inside `assets` and put all `.ttf` files in there.

### How to Build

#### wasm
- `wasm-pack build --target web`
- Start a 'liveserver' from inside the `pkg` folder and open index.html in a browser.

#### locally
- `cargo build`
- `cargo run`
