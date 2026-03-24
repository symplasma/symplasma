To recursively and efficiently list files based on patterns or MIME types in Rust, the best approach is to combine a **high-performance directory walker** with a **specialized MIME/glob matcher**.

There is no single "standard" crate that does everything at peak performance, so the choice depends on whether you prioritize speed, parallelization, or sophisticated pattern matching.

### 1. Best for Performance: `ignore` + `mimetype-detector`

This is the "pro-level" combination used for tools like `ripgrep`.

- **`ignore`**: This is arguably the fastest recursive directory walker in the Rust ecosystem. It is parallel by default and handles `.gitignore` files, hidden files, and symbolic links efficiently.
- **`mimetype-detector`**: A newer, extremely fast, zero-dependency crate that detects MIME types by reading only the first \~3KB of a file. It is faster than `tree_magic` and supports over 500 formats.

**Why this combination?**

You use `ignore` to traverse the filesystem at maximum speed and then apply a filter using `mimetype-detector` to check the content of files that passed your path-based patterns.

### 2. Best for Sophisticated Globs: `wax`

If your patterns are complex (e.g., you need features like "any path except X" or specific brace expansions), **`wax`** is the modern choice.

- **Features**: Unlike older glob crates, `wax` provides an integrated `.walk()` method. It is highly opinionated about portability and performance.
- **Negations**: It has first-class support for negations (e.g., `**/*.rs` but not `**/tests/**`), which is often faster than walking everything and filtering manually.

### 3. Best for Content-Based Filtering (MIME)

If your primary requirement is "find all files that are actually images" (regardless of extension), you need a "magic number" sniffer:

| Crate | Strength | Best Use Case |
|:-----------------------|:-----------------------|:-----------------------|
| **`mimetype-detector`** | **Performance.** Zero dependencies, thread-safe, and very high format coverage (\~550). | High-speed server-side processing or CLI tools. |
| **`tree_magic_mini`** | **Accuracy.** Uses a hierarchical tree of MIME types to minimize checks (e.g., if it's not a ZIP, don't check if it's a DOCX). | When you need to distinguish between very similar container formats. |
| **`infer`** | **Lightweight.** Small, simple, and has no dependencies. | Small binaries or `no_std` environments. |

### 4. Comparison Summary

| Goal                  | Recommended Crates                       |
|:----------------------|:-----------------------------------------|
| **Maximum Speed**     | `ignore` (parallel traversal)            |
| **Complex Patterns**  | `wax` or `globset`                       |
| **MIME by Extension** | `mime_guess` (instant, no I/O)           |
| **MIME by Content**   | `mimetype-detector` (reads file headers) |

### Implementation Example (Fast & Recursive)

Here is how you would combine `ignore` and `mimetype-detector` for an efficient, parallel search:

``` rust
use ignore::WalkBuilder;
use mimetype_detector::detect_file;

fn main() {
    // WalkBuilder allows for parallel, efficient recursion
    let walker = WalkBuilder::new("./").build();

    for result in walker {
        if let Ok(entry) = result {
            let path = entry.path();
            
            // 1. Efficiently filter by pattern (e.g., only files)
            if path.is_file() {
                // 2. Efficiently detect MIME type by reading only the header
                if let Ok(mime) = detect_file(path) {
                    if mime.to_string() == "image/jpeg" {
                        println!("Found JPEG: {:?}", path);
                    }
                }
            }
        }
    }
}
```

### Pro-Tip: Avoid Double-Reading

For maximum efficiency, if you are searching for specific text *and* a MIME type, use `ignore` to find the path, then read the first few KB for the MIME type, and only if that matches, read the full file for your pattern. This prevents expensive disk I/O on files that don't match your criteria.
