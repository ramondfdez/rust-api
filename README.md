# 🚀 Rust Todo API

Welcome to the **Rust Todo API**! This is a lightweight, efficient API built using Rust and the Warp framework, designed for managing todo tasks with a MongoDB backend. 

## 🌟 Features

- **RESTful API**: Supports standard HTTP methods (GET, POST, PATCH, DELETE).
- **Health Checker**: Quick health check endpoint to ensure the API is running smoothly.
- **CORS Support**: Configurable CORS support for seamless integration with frontend applications.
- **MongoDB Integration**: Efficiently manages todo tasks using a MongoDB database.

## 📦 Installation

To get started with the Rust Todo API, follow these steps:

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/rust-todo-api.git
cd rust-todo-api
```

### 2. Install Rust

Ensure you have Rust installed on your machine. If not, you can install it by following the instructions on the official Rust website.

###  3. Set Up Dependencies

Make sure you have the required dependencies in your Cargo.toml file:

```bash
[dependencies]
warp = "0.3"
tokio = { version = "1", features = ["full"] }
mongodb = "2.0"
pretty-env-logger = "0.4"
chrono = "0.4"
```

###  4. Start MongoDB

If you haven't already, install and start MongoDB. You can follow the instructions on the MongoDB installation guide.

###  5. Run the API

```bash
cargo run
```

Your API will start on http://localhost:8000.

🛠️ Usage
Health Check Endpoint

Check if the API is running smoothly:

    GET /api/healthchecker

Todo Routes
Create a Todo

    POST /api/todos

json

{
  "title": "Learn Rust",
  "content": "Study Rust programming language.",
  "completed": false
}

Get Todos

    GET /api/todos?page=1&limit=10

Get a Todo by ID

    GET /api/todos/{id}

Edit a Todo

    PATCH /api/todos/{id}

json

{
  "title": "Learn Rust (Updated)",
  "completed": true
}

Delete a Todo

    DELETE /api/todos/{id}

📖 Documentation

For detailed information on how to use the API, refer to the API Documentation.
🧑‍💻 Contributing

Contributions are welcome! If you'd like to contribute, please fork the repo and submit a pull request.
🙏 Acknowledgments

    Rust Programming Language
    Warp
    MongoDB
    Tokio

📝 License

This project is licensed under the MIT License - see the LICENSE file for details.
