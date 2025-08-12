# Knot Space

Online repository server for Knot packages.

## Features

- Package registry API
- Health check endpoint
- JSON-based package metadata
- RESTful API design

## API Endpoints

- `GET /` - Service information
- `GET /health` - Health check
- `GET /packages` - List all packages
- `GET /packages/<name>` - Get specific package
- `POST /packages` - Create new package

## Development

```bash
# Run the server
cargo run

# The server will start on http://localhost:8000
```

## Package Format

```json
{
  "name": "package-name",
  "version": "1.0.0",
  "team": "optional-team-name",
  "description": "Package description",
  "download_url": "https://example.com/package.tar.gz"
}
```