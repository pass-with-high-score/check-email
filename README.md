## Project Overview

**Project Name**: check-email

**Description**: This project is a Rust-based web service that checks if an email address is reachable. It uses the
`actix-web` framework for the web server and the `check-if-email-exists` crate to verify email addresses.

## Getting Started

### Prerequisites

- Docker
- Docker Compose

### Building and Running the Project

1. **Clone the repository**:
   ```sh
   git clone https://github.com/pass-with-high-score/check-email.git
   cd check-email
   ```

2. **Build and run the Docker container**:
   ```sh
   docker-compose build
   docker-compose up
   ```

3. The application will be available at `http://127.0.0.1:8080`.

## Project Structure

- `src/main.rs`: Entry point of the application.
- `src/middleware/mod.rs`: Middleware for logging.
- `src/models/mod.rs`: Data models for the application.
- `src/routers/mod.rs`: Route configurations.
- `src/routers/validate.rs`: Route handler for email validation.
- `src/models/validate.rs`: Data models for email validation.

## API Documentation

### Endpoint: `/check-email`

**Method**: POST

**Description**: Validates if an email address is reachable.

**Request Body**:

```json
{
  "email": "example@example.com"
}
```

**Response**:

- **Success** (HTTP 200):
  ```json
  {
    "input": "example@example.com",
    "is_reachable": true
  }
  ```
- **Failure** (HTTP 400):
  ```json
  {
    "error": "Invalid email format"
  }
  ```

### Data Models

#### CheckEmailRequest

- **email**: `String` - The email address to be validated. Must be a valid email format.

#### CheckEmailOutputResponse

- **input**: `String` - The input email address.
- **is_reachable**: `bool` - Indicates if the email address is reachable.

## Dependencies

- `actix-web`: Web framework for Rust.
- `log`: Logging library.
- `serde`: Serialization and deserialization library.
- `validator`: Validation library.
- `futures`: Asynchronous programming library.
- `actix-service`: Service trait for Actix.
- `check-if-email-exists`: Library to check if an email address exists.
