# Rust GB Docs

## Running the project locally

1. cd into the docs directory `cd docs/opcodes`
2. Run update-supported.sh from the root of the project
   `./update-supported.sh`
3. Build the project: `elm make --output=js/elm.js src/Main.elm`
4. Run using elm reactor: `elm reactor`
5. Navigate to `localhost:8000` and view `index.html`
