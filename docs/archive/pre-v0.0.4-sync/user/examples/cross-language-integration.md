---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/examples/cross-language-integration.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Cross-Language Integration Examples

**Version:** 0.0.2  
**Working Code:** Complete Implementations
**Last Updated:** 2025-12-02

---

## Example 1: Python → Rust Matrix Multiplication

Demonstrates calling Python for data generation and Rust for high-performance computation.

### `MatrixMultiply.pg`

```polyglot
[@] @Local::MatrixExample:1.0.0.0
[X]

[#] #Matrix
[<] <rows:pg.int
[<] <cols:pg.int
[<] <data:pg.array{pg\float
[X]

[|] |GenerateMatrices
[i] .size:pg.int
[t] |T.Call
[o] .matrix_a: #Matrix
[o] .matrix_b: #Matrix

[W] |W.RT.Python3.14

[r] |RT.Python.Run.File
[<] <file:pg.path << \\FileDir\\python\\generate_matrices.py
[<] <input:pg.serial << {.size
[>] >output:pg.serial >> .result

[r] .matrix_a: #Matrix << #Matrix {
[*] <rows:pg.int << .result.matrix_a.rows,
[*] <cols:pg.int << .result.matrix_a.cols,
[*] <data:pg.array{pg\float << .result.matrix_a.data
[*] 

[r] .matrix_b: #Matrix << #Matrix {
[*] <rows:pg.int << .result.matrix_b.rows,
[*] <cols:pg.int << .result.matrix_b.cols,
[*] <data:pg.array{pg\float << .result.matrix_b.data
[*] 

[o] .matrix_a: #Matrix
[o] .matrix_b: #Matrix
[X]

[|] |MultiplyMatrices
[i] .matrix_a: #Matrix
[i] .matrix_b: #Matrix
[t] |T.Call
[o] .result: #Matrix

[W] |W.RT.Rust1.8

[r] |RT.Rust.Run.Function
[<] <function:pg.string << "multiply::multiply_matrices"
[<] <args:pg.serial << {.matrix_a, .matrix_b
[>] >output:pg.serial >> .product_data

[r] .result: #Matrix << #Matrix {
[*] <rows:pg.int << .product_data.rows,
[*] <cols:pg.int << .product_data.cols,
[*] <data:pg.array{pg\float << .product_data.data
[*] 

[o] .result: #Matrix
[X]

[|] |MatrixWorkflow
[i] .size:pg.int <~ 100
[t] |T.Call
[o] .product: #Matrix

[r] |GenerateMatrices
[<] <size:pg.int << .size
[>] >matrix_a: #Matrix >> .a
[>] >matrix_b: #Matrix >> .b

[r] |MultiplyMatrices
[<] <matrix_a: #Matrix << .a
[<] <matrix_b: #Matrix << .b
[>] >result: #Matrix >> .product

[o] .product: #Matrix
[X]
```

---

## Example 2: Node → Go API Processing

Demonstrates Node.js validation and Go backend processing.

### `APIWorkflow.pg`

```polyglot
[@] @Local::APIExample:1.0.0.0
[X]

[#] #APIRequest
[<] <method:pg.string
[<] <endpoint:pg.string
[<] <payload:pg.serial
[X]

[|] |ValidateRequest
[i] .request: #APIRequest
[t] |T.Call
[o] .valid:pg.bool
[o] .errors:pg.array{pg\string

[W] |W.RT.Nodejs

[r] |RT.Nodejs.Run.File
[<] <file:pg.path << \\FileDir\\nodejs\\validate.js
[<] <input:pg.serial << {.request
[>] >output:pg.serial >> .result

[r] .valid:pg.bool << .result.valid
[r] .errors:pg.array{pg\string << .result.errors

[o] .valid:pg.bool
[o] .errors:pg.array{pg\string
[X]

[|] |ProcessRequest
[i] .request: #APIRequest
[t] |T.Call
[o] .response:pg.serial

[W] |W.RT.Go1.22

[r] |RT.Go.Run.Function
[<] <function:pg.string << "main.ProcessRequest"
[<] <args:pg.serial << {.request
[>] >output:pg.serial >> .response

[o] .response:pg.serial
[X]

[|] |APIWorkflow
[i] .request: #APIRequest
[t] |T.HTTP.POST
[<] <path:pg.string << "/api/process"
[o] .result:pg.serial
[o] !ValidationError

[r] |ValidateRequest
[<] <request: #APIRequest << .request
[>] >valid:pg.bool >> .is_valid
[>] >errors:pg.array{pg\string >> .validation_errors

[?] .is_valid =? #Boolean.True
[~][r] |ProcessRequest
[~][<] <request: #APIRequest << .request
[~][>] >response:pg.serial >> .result
[~][o] .result:pg.serial
[~]

[?] *?
[~][o] !ValidationError
[~]
[X]
```

---

## Key Takeaways

1. **Type Conversion** - Polyglot automatically handles JSON serialization
2. **FFI Abstraction** - No complex FFI code needed
3. **Performance** - Use Rust for CPU-intensive tasks
4. **Validation** - Node.js excels at schema validation

---

**Next:** [Automation Workflows →](automation-workflows.md
