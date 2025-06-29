# 🚀 Fibonacci Calculator API (Rust + Vercel)

A blazing-fast, serverless Fibonacci calculator built in Rust and deployed on Vercel. This API computes Fibonacci numbers up to 1000 using arbitrary-precision arithmetic, and is designed for reliability, scalability, and developer-friendliness.

---

## ✨ Features

- **Ultra-fast**: Powered by Rust and async execution
- **Big Number Support**: Uses `num-bigint` for huge Fibonacci numbers
- **Serverless**: Deploys instantly on Vercel, scales automatically
- **RESTful JSON API**: Clean, predictable responses
- **CORS Enabled**: Ready for frontend integration
- **Debug Info**: Each response includes helpful debug and usage info
- **Input Safety**: Caps input at 1000 to prevent abuse

---

## 🛠️ Tech Stack

- **Language**: Rust (2021 Edition)
- **Serverless Platform**: Vercel
- **Big Numbers**: [`num-bigint`](https://crates.io/crates/num-bigint)
- **Async Runtime**: [`tokio`](https://crates.io/crates/tokio)
- **JSON**: [`serde_json`](https://crates.io/crates/serde_json)
- **Time**: [`chrono`](https://crates.io/crates/chrono)

---

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rust-lang.org/tools/install)
- [Vercel CLI](https://vercel.com/docs/cli)
- [Git](https://git-scm.com/)

### Local Development

> **Note:** This is a Vercel serverless function. You cannot use `cargo run` directly.

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd my-rs-vercel-api
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Start local dev server with Vercel**
   ```bash
   npm i -g vercel
   vercel dev
   ```

4. **Test the API locally**
   ```bash
   curl http://localhost:3000/api/20
   ```

### Deploy to Vercel

1. **Login to Vercel**
   ```bash
   vercel login
   ```

2. **Deploy**
   ```bash
   vercel --prod
   ```

---

## 📚 API Documentation

### Base URL

```
https://<your-vercel-app>.vercel.app/api/[n]
```

### Endpoint

**GET** `/api/[n]`

- `n` (integer, optional): The Fibonacci number to calculate (default: 10, max: 1000)

#### Example Request

```bash
curl "https://<your-vercel-app>.vercel.app/api/20"
```

#### Example Response

```json
{
  "fibonacci": "6765",
  "n": 20,
  "timestamp": "2024-01-15T10:30:00Z",
  "status": "success",
  "debug": {
    "path": "/api/20",
    "query": "",
    "full_uri": "/api/20",
    "extraction_method": "path_analysis"
  },
  "usage": "To calculate Fibonacci of a different number, use: /api/25 (replace 25 with your desired number(integer))"
}
```

#### Response Fields

| Field        | Type    | Description                                 |
|--------------|---------|---------------------------------------------|
| fibonacci    | string  | The calculated Fibonacci number             |
| n            | integer | The input number used for calculation       |
| timestamp    | string  | ISO 8601 timestamp of the request           |
| status       | string  | Always "success" for valid requests         |
| debug        | object  | Debug information for troubleshooting       |
| usage        | string  | Instructions for using the API              |

#### Error Handling

- **Invalid Input**: Non-numeric values default to 10
- **Large Numbers**: Values > 1000 are capped at 1000
- **Missing Parameters**: Defaults to Fibonacci(10)

---

## 🏗️ Project Structure

```
my-rs-vercel-api/
├── api/
│   └── [n].rs           # Main API handler (dynamic route)
├── Cargo.toml           # Rust dependencies and configuration
├── vercel.json          # Vercel deployment configuration
└── README.md            # This file
```

---

## 🔧 Configuration

### Cargo.toml

```toml
[package]
name = "my-rust-vercel-api"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["macros"] }
serde_json = { version = "1", features = ["raw_value"] }
vercel_runtime = { version = "1" }
chrono = { version = "0.4", features = ["serde"] }
num-bigint = "0.4.6"
num-traits = "0.2"

[[bin]]
name = "fibonacci"
path = "api/[n].rs"
```

### vercel.json

```json
{
  "functions": {
    "api/[n].rs": {
      "runtime": "vercel-rust@4.0.9"
    }
  }
}
```

---

## 🧪 Testing

### Manual Testing

```bash
# Default (Fibonacci 10)
curl "https://<your-vercel-app>.vercel.app/api/"

# Specific number
curl "https://<your-vercel-app>.vercel.app/api/30"

# Large number (will be capped at 1000)
curl "https://<your-vercel-app>.vercel.app/api/1500"

# Invalid input (will default to 10)
curl "https://<your-vercel-app>.vercel.app/api/abc"
```

---

## 🔍 Algorithm Details

### Fibonacci Calculation

The API uses an **iterative algorithm** with O(n) time complexity and O(1) space complexity:

```rust
fn calculate_fibonacci(n: u64) -> BigUint {
    if n == 0 { return BigUint::from(0u32); }
    if n == 1 { return BigUint::from(1u32); }
    
    let mut a = BigUint::from(0u32);  // F₀
    let mut b = BigUint::from(1u32);  // F₁
    
    for _ in 2..=n {
        let next = &a + &b;  // Fₙ = Fₙ₋₁ + Fₙ₋₂
        a = b;
        b = next;
    }
    
    b
}
```

### BigUint Benefits

- **No Overflow**: Can handle numbers larger than 2⁶⁴
- **Precise**: No floating-point precision issues
- **Efficient**: Optimized for large number arithmetic

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [Vercel](https://vercel.com) for the serverless platform
- [Rust](https://rust-lang.org) for the programming language
- [num-bigint](https://crates.io/crates/num-bigint) for big integer support

---

**Made using Rust and Vercel**