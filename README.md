# Simple example that integrate rust lang with next js - ChatGPT

Rust and Next.js can work together but not in a direct way like you might think. Instead, we can compile Rust code to WebAssembly (Wasm), then use that Wasm module in our Next.js application.

Let's create a simple example:

First, you need to install the following:

1. Node.js & npm (https://nodejs.org/)
2. Rust & Cargo (https://www.rust-lang.org/tools/install)
3. wasm-pack (https://rustwasm.github.io/wasm-pack/installer/)

Next, let's create a new Rust library that we will compile to WebAssembly:

1. In your terminal, run the following commands:

```bash
$ cargo new --lib rust-wasm-lib
$ cd rust-wasm-lib
```

2. Replace the contents of `src/lib.rs` with:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

3. Compile this library to WebAssembly with:

```bash
$ wasm-pack build --target web
```

This will create a new `pkg` directory containing the compiled Wasm and some generated JavaScript to help use it.

Next, let's create a new Next.js app and use this Wasm module:

1. In another terminal, outside of your Rust project, run:

```bash
$ npx create-next-app@latest rust-wasm-next
$ cd rust-wasm-next
```

2. Install the `@wasmjs/wasm-webpack-plugin` plugin to enable Webpack to understand `.wasm` files:

```bash
$ npm install --save @wasmjs/wasm-webpack-plugin
```

3. In `next.config.js` add the plugin to Webpack's configuration:

```javascript
const WasmPackPlugin = require('@wasmjs/wasm-webpack-plugin');

module.exports = {
  webpack(config, options) {
    config.experiments = { asyncWebAssembly: true };
    config.plugins.push(
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, '../rust-wasm-lib'),
      })
    );

    return config;
  },
};
```

4. In the `pages/index.js` file, import and use the Wasm module:

```javascript
import { useEffect, useState } from 'react';
import { add } from '../rust-wasm-lib/pkg/rust_wasm_lib';

export default function Home() {
  const [result, setResult] = useState(0);

  useEffect(() => {
    setResult(add(2, 3));
  }, []);

  return (
    <div>
      <h1>Rust + Next.js</h1>
      <p>2 + 3 = {result}</p>
    </div>
  );
}
```

5. Run your Next.js app with `npm run dev`. You should see "2 + 3 = 5" displayed on the page.

This is a basic example of using Rust with Next.js through WebAssembly. In reality, you would probably use more complex Rust code and handle loading the Wasm module asynchronously.

## References

[rustwasm manual setup](https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/getting-started/manual-setup.html)

## Found errors:

### building for web the rust lib

`$wasm-pack build --target web`

error:

```ssh
$ wasm-pack build --target web

Error: crate-type must be cdylib to compile to wasm32-unknown-unknown. Add the following to your Cargo.toml file:

[lib]
crate-type = ["cdylib", "rlib"]
Caused by: crate-type must be cdylib to compile to wasm32-unknown-unknown. Add the following to your Cargo.toml file:

[lib]
crate-type = ["cdylib", "rlib"]

```

solution:

add to Cargo.toml file this line

```
[lib]
crate-type = ["cdylib", "rlib"]
```

### first start the nextjs

`$ yarn dev`

<b>error:</b>

```
Error: 'entryOptions.layer' is only allowed when 'experiments.layers' is enabled
```

solution: update `next.config.js` with adding `layers: true`

```
config.experiments = { asyncWebAssembly: true, layers: true };
```

### wasm-bindgen output fail

`pkg/rust_wasm_lib.js`

```javascript
/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function sum(a, b) {
  // this is the original code from the wasm-bindgen output
  // const ret = wasm.sum(a, b);
  // return ret;

  // this is the MANUAL code that I added to make it work
  return a + b;
}
```
