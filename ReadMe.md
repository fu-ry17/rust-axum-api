# Rust API with Axum

A simple Rust API using the Axum framework to manage a collection of blogs. The API allows you to perform basic CRUD operations on the blogs.

## Endpoints

- `GET /`: Get the root endpoint.
- `GET /blog`: Get all blogs.
- `POST /blog`: Create a new blog.
- `DELETE /blog/:id`: Delete a blog by ID.
- `PATCH /blog/:id`: Update a blog by ID.

## Prerequisites

- Rust: Ensure you have Rust and Cargo installed. You can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
- PostgreSQL: Make sure you have PostgreSQL installed and running.

## Installation

1. Clone the repository.

```bash
git clone https://github.com/fu-ry17/rust-axum-api.git
cd rust-axum-api
```

2. Create a `.env` file and add your PostgreSQL database connection string.

```plaintext
DATABASE_URL=postgres://username:password@localhost/your_database_name
```

3. Build and run the API.

```bash
cargo run
```

## Usage

### `GET /`

Get the root endpoint.

### `GET /blog`

Get all blogs.

### `POST /blog`

Create a new blog.

**Request Body**

```json
{
  "title": "Your Blog Title",
  "description": "Your Blog Description"
}
```

### `DELETE /blog/:id`

Delete a blog by ID.

### `PATCH /blog/:id`

Update a blog by ID.

**Request Body**

```json
{
  "title": "Updated Blog Title",
  "description": "Updated Blog Description"
}
```

## Data Structures

### Blog

A blog entity with the following fields:

- `id`: An integer representing the blog's unique identifier.
- `title`: A string representing the blog's title.
- `description`: A string representing the blog's description.
- `created_at`: A chrono::NaiveDateTime representing the blog's creation date.
- `updated_at`: A chrono::NaiveDateTime representing the blog's last update date.

### CreateBlog

A data structure used for creating a new blog with the following fields:

- `title`: A string representing the blog's title.
- `description`: A string representing the blog's description.

### BlogId

A data structure used to extract the blog ID from the URL path.

## Contributing

Contributions are welcome! If you find any issues or want to add new features, feel free to open a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

For any questions or feedback, please reach out to:

- Email: furybrian175@gmail.com
- Project Repository: [https://github.com/fu-ry17/rust-axum-api](https://github.com/fu-ry17/rust-axum-api)

---

Happy coding!
