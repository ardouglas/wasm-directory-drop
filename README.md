# Drag 'N Drop Examples
This repo provides both a working js example and a non-working rust example of working with the experimental `FileSystemDirectoryHandle` from a directory drag and drop event.

Both examples are focused exclusively on handling directory events and ignore paths where the input is a single file.


## JS
To run the js example, you will need a static webserver such as the python simple webserver to correctly load the web worker.  To serve the page:

``` bash
cd js-example
python3 -m http.server 9000
```

Then you should be able to access the pages correctly at localhost:9000

## Rust

To run the rust example, you will need the rust compiler toolchain with the wasm32-unknown-unknown target installed, wasm-pack, and trunk.


To run the rust example, run`trunk serve` from the top of the repo and open a browser to port 8080.  Drag and drop a directory onto the page and view the console output.


## Observations
Locally I have only tested with a small directory (five files/dirs nested inside the top level).  In the js code, as expected, I see the worker code print to the console five times.  The (I believe) equivalent rust worker code seems to print to the console hundreds of thousands of times. 