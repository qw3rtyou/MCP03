# MCP03 - Rust CMS

A modern Content Management System built with Rust and Actix-web.

## Features

- User Authentication
- Content Management
- Media Upload
- API Endpoints
- PostgreSQL Database

## Getting Started

1. Clone the repository
2. Install Docker and Docker Compose
3. Run `docker-compose up --build`
4. Access the CMS at http://fooo1.com:5555

## API Documentation

### Authentication
- POST /api/auth/register
- POST /api/auth/login

### Content
- GET /api/content
- POST /api/content
- PUT /api/content/:id
- DELETE /api/content/:id

### Media
- POST /api/media/upload
- GET /api/media/:id

## Development

```bash
# Run in development mode
docker-compose up --build

# Run tests
cargo test
```