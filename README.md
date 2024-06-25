###What is SHA256?
SHA256 (Secure Hash Algorithm 256-bit) is a cryptographic hash function that generates a 256-bit (32-byte) hash value from an input. It is widely used in security applications and protocols, including SSL/TLS and cryptographic signatures.

### Purpose of the Project
The main goal of the "rust-sha256-cracker" project is to break the security of SHA256 hashes by trying all possible combinations of inputs until the correct one is found. This technique is known as brute force attack.

### Why Rust?
Rust is chosen for its powerful combination of performance and safety features:
- **Performance:** Rust is a systems programming language that compiles to native code, allowing for high-speed execution comparable to C or C++.
- **Safety:** Rust's ownership model ensures memory safety without needing a garbage collector, preventing common bugs such as null pointer dereferencing and buffer overflows.
- **Concurrency:** Rust has built-in support for safe concurrency, making it easier to write multi-threaded applications that can utilize multiple CPU cores effectively.

### Components of the Project

1. **Hash Cracking Logic:** 
   - The core of the application is the algorithm that performs the brute force attack. It iteratively generates possible input strings, computes their SHA256 hash, and compares it with the target hash.
   - Given the vast number of possible inputs, the algorithm needs to be highly optimized for speed.

2. **Multithreading:**
   - To enhance performance, the application can leverage Rust’s concurrency capabilities by dividing the work among multiple threads.
   - Each thread works on a different range of possible inputs, significantly reducing the time required to find the correct input.

3. **Input Generation:**
   - The application must generate all possible combinations of characters to test as potential inputs. This could be limited to certain character sets (e.g., alphanumeric) to reduce the search space.

4. **Comparison and Output:**
   - For each generated input, the application computes its SHA256 hash and checks if it matches the target hash.
   - If a match is found, the application outputs the original input string.
