# 📮 puttr

a silly little web service.

`puttr` allows you to send it a PUT request whose key is `content`, and this will write the associated value to a local file. Requests must include a valid authentication token. Files are stored in a configurable directory and organized by year-month with timestamps. The file extension is determined by the `Content-Type` header of your request.

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

## Configuration

puttr reads configuration from a `puttr.toml` file in the working directory. Currently, the only configurable option is:

```toml
[storage]
upload_dir = "uploads"
```

This specifies the directory where uploaded files will be stored. The directory will be automatically created if it doesn't exist. Files are organized in a year-month subdirectory structure within this directory.

**Security Note:** The `upload_dir` value is validated on startup:
- Must be a relative path (no leading `/` or `~`)
- Cannot contain `..` path traversal sequences
- Cannot be empty
- Cannot contain null characters

This ensures that uploaded files are stored only within the intended upload directory and prevents directory traversal attacks.

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

The file extension is determined by the `Content-Type` header. For example:
- `application/json` → `.json`
- `image/png` → `.png`
- `text/plain` → `.txt` (default)

**Using curl with JSON:**

```bash
curl -X PUT \
  -H "Content-Type: application/json" \
  -H "Authorization: Token WW91IGZvdW5kIGEgdG9rZW4h" \
  -d "content={\"key\": \"value\"}" \
  http://localhost:3000/data
```

**Using curl with plain text:**

```bash
curl -X PUT \
  -H "Content-Type: text/plain" \
  -H "Authorization: Token WW91IGZvdW5kIGEgdG9rZW4h" \
  -d "content=hello world" \
  http://localhost:3000/data
```

**Using httpie:**

```bash
http PUT http://localhost:3000/data \
  Authorization:"Token WW91IGZvdW5kIGEgdG9rZW4h" \
  Content-Type:"application/json" \
  content='{"key": "value"}'
```

The content will be written to `uploads/YYYY-MM/data-<timestamp>-<token>.<ext>` where `<ext>` is determined by the Content-Type header.

### Error Handling

- **Missing Token**: If you don't include an `Authorization` header, you'll receive a `401 Unauthorized` response.
- **Expired Token**: If your token has expired (after 5 minutes), you'll receive a `401 Unauthorized` response. Request a new token and try again.
- **Invalid Content**: If the `content` field is empty or missing, you'll receive a `404 Not Found` response.

### Supported Content Types

The following Content-Type headers are supported for automatic file extension mapping:

**Application Types:**
- `application/json` → `.json`
- `application/xml` → `.xml`
- `application/pdf` → `.pdf`
- `application/zip` → `.zip`
- `application/gzip` → `.gz`

**Text Types:**
- `text/plain` → `.txt`
- `text/html` → `.html`
- `text/css` → `.css`
- `text/javascript` → `.js`
- `text/csv` → `.csv`
- `text/markdown` → `.md`

**Image Types:**
- `image/png` → `.png`
- `image/jpeg` → `.jpg`
- `image/gif` → `.gif`
- `image/webp` → `.webp`
- `image/svg+xml` → `.svg`

**Audio Types:**
- `audio/mpeg` → `.mp3`
- `audio/wav` → `.wav`
- `audio/flac` → `.flac`

**Video Types:**
- `video/mp4` → `.mp4`
- `video/webm` → `.webm`
- `video/quicktime` → `.mov`

If you use an unsupported Content-Type, the default `.txt` extension will be used.

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


