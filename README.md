# Chop Shop

This project demonstrates a simple web application with intentional security vulnerabilities including SQL injection and XSS (Cross-Site Scripting).

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

## Security Vulnerabilities for Educational Purposes

This application contains several intentional security vulnerabilities for educational purposes:

### 1. SQL Injection

To test the SQL injection vulnerability, you can use a web browser or a tool like `curl` to send a request with a malicious payload.

#### Example SQL Injection Test

1. **Open a web browser** and navigate to the following URL:
   ```
   http://127.0.0.1:8080/search?query='; INSERT INTO items (name, price, estimated_ship_date, supplier) VALUES ('Coconut Oil Caliper', 99.99, '2023-12-01', 'Fake Supplier'); --
   ```

2. **Use `curl` to send a request**:
   ```sh
   curl "http://127.0.0.1:8080/search?query='; INSERT INTO items (name, price, estimated_ship_date, supplier) VALUES ('Coconut Oil Caliper', 99.99, '2023-12-01', 'Fake Supplier'); --"
   ```

This should demonstrate the SQL injection vulnerability by inserting a fake item into the `items` table. If the injection is successful, the fake item "Coconut Oil Caliper" should be added to the database.

### 2. Cross-Site Scripting (XSS)

The search functionality is vulnerable to XSS attacks as it doesn't properly sanitize user input.

#### Testing XSS Vulnerability

1. **Basic Alert Test**:
   Navigate to the search page and enter the following in the search box:
   ```
   <script>alert('XSS vulnerability detected!')</script>
   ```
   When you submit the search, an alert should pop up, demonstrating that arbitrary JavaScript can be executed.

2. **Persistent XSS Attack**:
   The application logs search queries without proper sanitization. Try entering:
   ```
   <script>console.log(document.cookie)</script>
   ```
   This would log any cookies to the browser console, demonstrating how attackers could steal sensitive information.

3. **More Advanced Payload**:
   For a more visual demonstration:
   ```
   <img src="x" onerror="alert('Image XSS attack vector')">
   ```
   This executes JavaScript when the image fails to load.

### 3. Sensitive Information Logging

The Contact page demonstrates insecure logging of sensitive information:

1. Submit the contact form with some "sensitive" information
2. The application logs this data in plain text to a log file
3. Check the log.txt file to see the unencrypted sensitive data

## Other Features

- **Parts Search**: Search for auto parts in the inventory
- **Show All Parts**: View the complete inventory at once
- **Sell Parts**: Add new parts to the inventory
- **Contact Form**: Submit inquiries (demonstrates insecure logging)

## Important Note

This project is for educational purposes only. Do not use this code in production environments as it contains intentional security vulnerabilities designed for security awareness training.
