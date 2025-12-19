# puttr

a silly little web service.

`puttr` allows you to send it a PUT request whose key is `content`, and this will write the associated value to a local test file. Requests must include a valid authentication token.

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

## API Usage

### 1. Request a Token

Before you can send data to the `/data` endpoint, you need to obtain an authentication token. Tokens are valid for 5 minutes.

**Using curl:**

```bash
curl http://localhost:3000/token
```

**Using httpie:**

```bash
http GET http://localhost:3000/token
```

Both commands will return a base64-encoded token, e.g.:
```
WW91IGZvdW5kIGEgdG9rZW4h
```

### 2. Send Data with Authorization Token

Once you have a token, use it to send data to the `/data` endpoint via a PUT request. Include the token in the `Authorization: Token <value>` header.

**Using curl:**

```bash
curl -X PUT \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -H "Authorization: Token WW91IGZvdW5kIGEgdG9rZW4h" \
  -d "content=hello world" \
  http://localhost:3000/data
```

**Using httpie:**

```bash
http PUT http://localhost:3000/data \
  Authorization:"Token WW91IGZvdW5kIGEgdG9rZW4h" \
  content="hello world"
```

The content will be written to `data.txt` in the current directory.

### Error Handling

- **Missing Token**: If you don't include an `Authorization` header, you'll receive a `401 Unauthorized` response.
- **Expired Token**: If your token has expired (after 5 minutes), you'll receive a `401 Unauthorized` response. Request a new token and try again.
- **Invalid Content**: If the `content` field is empty or missing, you'll receive a `404 Not Found` response.

### Complete Example

```bash
# 1. Get a token
TOKEN=$(curl -s http://localhost:3000/token)

# 2. Send data with the token
curl -X PUT \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -H "Authorization: Token $TOKEN" \
  -d "content=My important data" \
  http://localhost:3000/data
```


