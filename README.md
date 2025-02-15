# node-macos-name-uuid

**node-macos-name-uuid:** A native Node.js module to get macOS computer name and the hardware UUID

This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Building node-macos-name-uuid

Building node-macos-name-uuid requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

To run the build, run:

```sh
$ npm run build
```

This command uses the [@neon-rs/cli](https://www.npmjs.com/package/@neon-rs/cli) utility to assemble the binary Node addon from the output of `cargo`.

## Example

After building node-macos-name-uuid, you can use it as a normal CommonJS module:

```js
const { getComputerName, getHardwareUuid } = require('node-macos-name-uuid');
console.log(getComputerName());
console.log(getHardwareUuid());
```

## Available Scripts

In the project directory, you can run:

#### `npm install`

Installs the project, including running `npm run build`.

#### `npm run build`

Builds the Node addon (`index.node`) from source, generating a release build with `cargo --release`.

Additional [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) arguments may be passed to `npm run build` and similar commands. For example, to enable a [cargo feature](https://doc.rust-lang.org/cargo/reference/features.html):

```
npm run build -- --feature=beetle
```

You can also specify `:x86_64` or `:arm64` to specify target arch.

#### `npm run debug`

Similar to `npm run build` but generates a debug build with `cargo`.

#### `npm test`

Runs the unit tests by calling `cargo test`. You can learn more about [adding tests to your Rust code](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) from the [Rust book](https://doc.rust-lang.org/book/).

## Project Layout

The directory structure of this project is:

```
node-macos-name-uuid/
├── Cargo.toml
├── README.md
├── src/
|   └── lib.rs
├── index.node
├── package.json
└── target/
```

| Entry          | Purpose                                                                                                                                  |
|----------------|------------------------------------------------------------------------------------------------------------------------------------------|
| `Cargo.toml`   | The Cargo [manifest file](https://doc.rust-lang.org/cargo/reference/manifest.html), which informs the `cargo` command.                   |
| `README.md`    | This file.                                                                                                                               |
| `src/`         | The directory tree containing the Rust source code for the project.                                                                      |
| `lib.rs`       | Entry point for the Rust source code.                                                                                                          |
| `index.node`   | The main module, a [Node addon](https://nodejs.org/api/addons.html) generated by the build and pointed to by `"main"` in `package.json`. |
| `package.json` | The npm [manifest file](https://docs.npmjs.com/cli/v7/configuring-npm/package-json), which informs the `npm` command.                    |
| `target/`      | Binary artifacts generated by the Rust build.                                                                                            |
