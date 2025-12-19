# puttr

a silly little web service.

`puttr` allows you to send it a PUT request whose key is `content`, and this will write the associated value to a local test file.

## Building

This is a Rust project that uses Cargo. To build:

```bash
cargo build
```

For a release build with optimizations:

```bash
cargo build --release
```

The compiled binary will be located at:
- Debug: `target/debug/puttr`
- Release: `target/release/puttr`

## Running

Start the web service:

```bash
cargo run
```

Or run the compiled binary directly:

```bash
./target/release/puttr
```

The service will start on `http://localhost:3000`.

### Usage

Visit `http://localhost:3000` in your browser to see the HTML instructions, or send a PUT request to `/data` with form data containing a `content` key:

```bash
curl -X PUT \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "content=hello world" \
  http://localhost:3000/data
```

The content will be written to `data.txt` in the current directory.


