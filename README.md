# Chop Shop

This project demonstrates a simple web application with a vulnerable SQL injection endpoint.

## Setup

### Prerequisites

- Rust and Cargo: Install from [rust-lang.org](https://www.rust-lang.org/)
- SQLite: Ensure SQLite is installed on your system

### Steps

1. **Clone the repository**:
   ```sh
   git clone <repository-url>
   cd chop-shop
   ```

2. **Set up the environment**:
   Create a `.env` file in the root of the project with the following content:
   ```plaintext
   DATABASE_URL=sqlite://chop_shop.db
   ```

3. **Install dependencies**:
   ```sh
   cargo build
   ```

4. **Run the setup script**:
   This script will create the database file, run the migration, and insert sample data.
   ```sh
   cargo run --bin setup
   ```

5. **Run the web server**:
   ```sh
   cargo run --bin chop-shop
   ```

## Testing SQL Injection

To test the SQL injection vulnerability, you can use a web browser or a tool like `curl` to send a request with a malicious payload.

### Example SQL Injection Test

1. **Open a web browser** and navigate to the following URL:
   ```
   http://127.0.0.1:8080/search?query='; INSERT INTO items (name, price, estimated_ship_date, supplier) VALUES ('Coconut Oil Caliper', 99.99, '2023-12-01', 'Fake Supplier'); --
   ```

2. **Use `curl` to send a request**:
   ```sh
   curl "http://127.0.0.1:8080/search?query='; INSERT INTO items (name, price, estimated_ship_date, supplier) VALUES ('Coconut Oil Caliper', 99.99, '2023-12-01', 'Fake Supplier'); --"
   ```

This should demonstrate the SQL injection vulnerability by inserting a fake item into the `items` table. If the injection is successful, the fake item "Coconut Oil Caliper" should be added to the database.

## Important Note

This project is for educational purposes only. Do not use this code in production environments as it contains intentional security vulnerabilities.
