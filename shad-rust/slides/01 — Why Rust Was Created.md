## Summary

The lecture explains Rust by comparing it with C++, Go, and Python. Rust tries to preserve the speed and low-level control of C/C++ while removing most undefined behavior from ordinary safe code through ownership/borrowing, lifetime checks, the type system, and explicit `unsafe` blocks.

Main idea: **dangerous code does not disappear in Rust, but it is isolated inside small `unsafe` sections, on top of which safe abstractions are built** (`Vec`, `Cell`, `RefCell`, `Mutex`, `Arc`, `rayon::join`, `thread::spawn`, etc.).

## Notes

### 00:00–03:00 — Course format and the main question

- Course format: lectures/seminars, homework after each class, automatic checks, possibly code review.
- Main question of the lecture: why do we need Rust if we already have C++, Go, and Python?

### 03:00–15:00 — Rust vs C++: why move beyond C++

- C++ is chosen for:
    - performance;
    - low-level resource control;
    - predictable data layout;
    - no garbage collector.
- C++ is still widely used in systems software:
    - databases: Google Borg/Spanner, Meta RocksDB/LogDevice, Yandex YTsaurus/ClickHouse/YDB;
    - runtimes: JVM, CLR;
    - browsers: Chrome, Firefox, Edge, Safari;
    - parts of operating systems.
- The main pain point in C++ is **undefined behavior**:
    - an error can be ignored with an unpredictable result;
    - behavior can depend on OS, architecture, or optimization flags;
    - the program can crash;
    - data can be corrupted;
    - security vulnerabilities can appear.
- C++ partially fights UB through RAII and static analysis, but that is not always enough.

### 15:00–20:00 — What Rust gives in terms of performance

- Rust aims for performance and control close to C++.
- **Zero-cost abstractions**: iterators, slices, and structs do not necessarily add function calls, allocations, or hidden pointers.
- Data layout is close to how structures are written in code: a nested structure can be stored directly inside the outer structure rather than through a pointer.
- The lecture shows a Bubble Sort CPU/memory usage study: Rust is close to C/C++ in CPU and memory usage, and much better than Python/Java in those microbenchmarks.

### 20:00–33:30 — Rust safety

- Approximate claim from the slide: as long as the code does not contain `unsafe`, it should not contain UB.
- `unsafe` does not make code automatically wrong; it gives C/C++-like “superpowers” and responsibility.
- Rust’s core model: write a small amount of `unsafe` inside a library, but expose a safe API that cannot be called in a way that causes UB.
- The compiler catches typical C++ bugs:
    - taking a reference to a `Vec` element and then calling `push`, which may reallocate the buffer;
    - a closure that may outlive a captured reference;
    - a data race when accessing `Cell` from multiple threads.
- Rust is not absolutely safe:
    - soundness holes exist;
    - `unsafe` inside libraries is not always formally verified;
    - a safe API with an incorrect `unsafe` implementation can still be a bug.
- A Rust CVE study: most memory-safety bugs are connected to `unsafe`, not to safe Rust itself.

### 33:30–42:00 — Rust vs Go

- Go is chosen for:
    - a simple language;
    - easy concurrency: goroutines/channels;
    - fast compilation;
    - decent performance.
- But Go does not close every safety problem:
    - data races are possible if synchronization is forgotten;
    - the Go memory model gives DRF-SC: if there is no data race, execution can be considered sequentially consistent;
    - nil remains a source of runtime bugs: nil map, nil interface, nil struct field.
- Rust enforces synchronization more strongly through types: shared mutable state requires things like `Arc`, `Mutex`, and `Sync`/`Send`-compatible types.
- Takeaway: Rust is safer and more expressive than Go, but it is also more complex and compiles more slowly.

### 42:00–46:40 — Rust vs Python

- Python is good for I/O-bound web applications, scripts, and ML/analytics.
- Rust can also be used for web applications and scripts:
    - simple Rust web services do not have to be much more complex than Python ones;
    - Rust scripts can be more reliable because of static typing and explicit error handling;
    - Rust integrates well with Python through PyO3/maturin-style tools.
- Takeaway: Rust is faster and more reliable than Python, but has a higher entry barrier.

### 47:00–52:00 — Where Rust is used

- Google/Android: Rust is used to reduce memory-safety risks, with some C/C++ code moving toward Rust.
- Linux: Rust for Linux adds Rust support to the kernel and drivers.
- Meta: Rust is used in infrastructure projects, especially source-control tooling.
- AWS: Rust is used in Firecracker, Bottlerocket, Nitro components, S3/EC2/CloudFront, and the Tokio ecosystem.
- Embedded: Rust can work without `std` and can run on microcontrollers; the lecturer mentions personal experience with ESP32.

## Code examples from the slides

### C++: out-of-bounds array access → UB

```cpp
std::array<int, 4> a = {1, 2, 3, 4};
int i, x;
std::cin >> i >> x;
a[i] = x;
```

What can happen:

- the error is ignored;
- the program crashes;
- an infinite loop appears;
- data corruption;
- security vulnerability.

### C++: dangling reference during range-for

```cpp
std::vector<std::string> createStrings();

for (char c : createStrings().at(0)) {
    // undefined behavior
}
```

Idea of the example: the temporary object may be destroyed before iteration over the string finishes.

### C++: invalidated iterator after `push_back`

```cpp
std::vector<std::string> strings = {"foo", "bar"};
auto it = strings.begin();
strings.push_back("baz");
std::cout << *it << std::endl;
```

`push_back` may reallocate the `vector` buffer, making the old iterator dangling.

### Rust: zero-cost loop over a slice

```rust
fn sum_squares(v: &[i32]) -> i32 {
    let mut res = 0;
    for x in v {
        res += x * x;
    }
    res
}
```

Variant using an iterator chain:

```rust
fn sum_squares(v: &[i32]) -> i32 {
    v.iter().map(|x| x * x).sum()
}
```

Takeaway: iterator abstractions do not have to introduce runtime overhead.

### Rust: data layout without extra indirection

```rust
struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

fn get_x(foo: &Foo) -> i32 {
    foo.bar.x
}
```

`Bar` is stored inside `Foo`, not as a separate heap object behind a pointer.

### Rust: borrow checker catches an invalidated reference

```rust
let mut v = vec![1, 2, 3];
let x = &v[0];
v.push(4);
println!("{}", x);
```

The compiler reports an error like `E0502`: `push` requires a mutable borrow, but the immutable borrow `x` is still alive.

### Rust: a closure can outlive a captured reference

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    |y| {
        x + y
    }
}
```

Idea from the slide: the closure may outlive the current function and a captured variable/reference. A typical fix is to explicitly capture the value with `move`:

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
```

### Rust: attempted data race with `Cell`

```rust
use std::cell::Cell;

let x = Cell::new(0);
rayon::join(
    || {
        x.set(3);
    },
    || {
        x.set(5);
    },
);
```

`Cell` is not `Sync`, so Rust will not safely share it between threads.

### Rust: `unsafe` inside a `Vec::push`-like implementation

```rust
pub fn push(&mut self, value: T) {
    let end = self.as_mut_ptr().add(self.len);
    unsafe {
        ptr::write(end, value);
    }
    self.len += 1;
}
```

Meaning: the standard library uses `unsafe`, but exposes a safe abstraction. A user of `Vec::push` should not be able to cause UB through the ordinary safe API.

### Rust async: safe shared state through `Arc<Mutex<_>>`

```rust
let data = Arc::new(Mutex::new(HashMap::new()));

tokio::spawn({
    let data = data.clone();
    async move {
        data.lock().unwrap().insert("foo", "bar");
    }
});

tokio::spawn({
    let data = data.clone();
    async move {
        data.lock().unwrap().insert("bar", "baz");
    }
});
```

Rust forces shared ownership and synchronization to be expressed explicitly.

### Go: data race that the compiler will not catch

```go
data := make(map[string]string)

go func() {
    data["foo"] = "bar"
}()

go func() {
    data["bar"] = "baz"
}()
```

In Go, this requires synchronization discipline from the programmer.

### Go: nullability is always nearby

```go
var data map[string]string // nil
data := make(map[string]string) // not nil

func GetReader() io.Reader { /* ... */ }
func GetClient() *Client { /* ... */ }

type MyStruct struct {
    reader io.Reader // private member CAN BE NIL!
}
```

Problem: nil can be hidden in maps, interfaces, or struct fields and then surface as a runtime panic.

### Go: RAII-like cleanup through `defer`, which must not be forgotten

```go
client := http.Client{}
resp, err := client.Get("https://google.com")
if err != nil {
    return nil, err
}
defer resp.Body.Close()
io.Copy(os.Stdout, resp.Body)
```

In Rust, similar resources are often released through `Drop` when a value goes out of scope.

### Rust web example

```rust
let app = Router::new()
    .route("/", get(root))
    .route("/users", post(create_user));

async fn root() -> &'static str {
    "Hello, World!"
}
```

This resembles `axum`: a compact routing DSL, async handlers, and statically typed responses.

### Rust script example with error context

```rust
cmd!(sh, "git push --force {STUDENT_REMOTE_NAME} HEAD:{branch}")
    .run()
    .with_context(|| format!("failed to push to branch \"{branch}\""))?;
```

Idea: Rust scripts can still be expressive while getting compile-time checks and useful error context.

### Rust + Python through PyO3

```rust
use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn string_sum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

Takeaway: Rust can be used as a fast and safe native module for Python.

## Links and artifacts

### Research / formal models

- [Ralf Jung — Understanding and Evolving the Rust Programming Language](https://research.ralfj.de/thesis.html) — dissertation on RustBelt, Stacked Borrows, and formalizing Rust.
- [RustBelt project](https://plv.mpi-sws.org/rustbelt/) — formal model of a Rust subset + soundness proofs for safe abstractions built on top of `unsafe`.
- [RustBelt: Securing the Foundations of the Rust Programming Language](https://plv.mpi-sws.org/rustbelt/popl18/paper.pdf) — POPL paper; Coq/Iris, verification of `Arc`, `Rc`, `Cell`, `RefCell`, `Mutex`, `RwLock`, `thread::spawn`, `rayon::join`.
- [Memory-Safety Challenge Considered Solved? An In-Depth Study with All Rust CVEs](https://arxiv.org/abs/2003.03296) — study of Rust memory-safety CVEs; useful for the claim that safe Rust is rarely the source of memory-safety bugs, while risk is concentrated around `unsafe`.
- [The Safety and Performance of Prominent Programming Languages](https://dblp.org/db/journals/ijseke/ijseke32.html) — Bugden & Alahmar, IJSEKE 2022; the lecture used a CPU/memory usage chart from this kind of comparison.

### Documentation for lecture topics

- [The Rust Programming Language — Unsafe Rust](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html) — what `unsafe` enables and which operations become available.
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — book about unsafe Rust and invariants of safe abstractions.
- [Rust Reference — Behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) — official list of UB in Rust.
- [The Go Memory Model](https://go.dev/ref/mem) — DRF-SC: data-race-free Go programs behave sequentially consistently.
- [PyO3](https://pyo3.rs/) — writing Python modules in Rust.
- [axum](https://github.com/tokio-rs/axum) — Tokio/hyper-based web framework, similar to the slide example.
- [rayon](https://github.com/rayon-rs/rayon) — data-parallelism library, `rayon::join`.
- [Tokio](https://tokio.rs/) — async runtime ecosystem.

### Where Rust is used

- [Rust for Linux](https://rust-for-linux.com/) and [Rust in the Linux kernel documentation](https://docs.kernel.org/rust/index.html).
- [AWS: Why AWS loves Rust](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust-and-how-wed-like-to-help/) — Firecracker, Bottlerocket, the Tokio ecosystem, and why AWS invests in Rust.
- [AWS: Why AWS is the Best Place to Run Rust](https://aws.amazon.com/blogs/devops/why-aws-is-the-best-place-to-run-rust/) — overview of Rust in AWS and infrastructure.
- [Firecracker](https://github.com/firecracker-microvm/firecracker) — microVM virtualization, Rust.
- [Bottlerocket](https://github.com/bottlerocket-os/bottlerocket) — AWS Linux-based container OS.
- [Meta engineering: A brief history of Rust at Meta](https://engineering.fb.com/2021/04/29/developer-tools/rust/) — Rust as a supported language at Meta.
- [Mononoke / Sapling source control](https://github.com/facebook/sapling) — Meta source-control tooling with a significant Rust codebase.
- [Google Security Blog: Rust in Android](https://security.googleblog.com/search/label/Rust) — Google posts about Rust and reducing memory-safety vulnerabilities in Android.
