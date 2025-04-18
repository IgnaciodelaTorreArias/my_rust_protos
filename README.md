# FFI: Exporting Rust Functions via C ABI with Protobuf-based Arguments and Results

This module explores the Foreign Function Interface (FFI) in Rust, focusing on:

- Exporting functions using the C ABI
- Passing function parameters and results using Protobufs

## Function Categories

The exported functions fall into two categories:

1. **Standalone Functions**: Simple functions that are not associated with any struct.
2. **Method Functions**: Methods of structs that are exported as C ABI-compatible functions.

Both types of functions **must** return an `i32` as the status code:

- `0`: Success
- `-1`: Result unavailable (e.g., unable to pass or decode results)
- `-2`, `-3`: Invalid arguments
- `-5`: Invalid instance pointer

### Extended Error Details

For even negative values, additional error details may be available. The specifics of how the details are passed will be explained later.

## Passing Arguments and Retrieving Results with Protobufs

### Standalone Function Example

A simple function that expects arguments may have the following signature:

```rust
pub extern "C" fn foo(ptr: *const u8, len: usize) -> i32
```

To call this function from other language:

1. Create a `foo_arguments` Protobuf message.
2. Encode `foo_arguments` into an array of bytes.
3. Get a pointer to the byte array and pass it to the function along with its length.
4. The function will process the byte array and return a status code.
extra. Depending on the language we need to make sure that the array isn't moved/freed/deleted when calling the function.

A function that returns a result may have the following signature:

```rust
pub extern "C" fn foo(out_ptr: *mut *mut u8, out_len: *mut usize) -> i32
```

To call this function from other language:

1. Create a pointer (`out_ptr`) and a length variable (`out_len`).
2. Pass the addresses of these variables to the function.
3. In Rust we will encode the `foo_results` Protobuf message into a `Vec<u8>` with the required capacity and make it ManuallyDrop.
4. In Rust we will make the output pointer (`out_ptr`) point to the newly created buffer, and set the length (`out_len`).
5. Once the results are retrieved, the buffer should be freed using a special `free_buffer` function.
extra. If an error occurs and we want to pass the details, instead of `foo_results` we replace it with and `Error` proto message.

### Combining Arguments and Results

When a function requires input arguments and returns results, both previously explained strategies can be combined.

## Method Functions (Methods of Structs)

Method functions require an associated struct, and there are two common strategies for mapping methods to C ABI functions:

### Strategy 1: Creating a New Struct Instance

In this approach, each function call creates a new instance of the struct.

- **Pros**: Safer, as Rust handles memory management.

- **Cons**: More overhead due to the need to create the struct instance for every method call.

### Strategy 2: Managing a Pointer to the Struct Instance

This approach works by passing a pointer to the struct instance, which is allocated on the heap.

- **Pros**: Less overhead, as you only pass the pointer to the struct and the method arguments.

- **Cons**: Unsafe. You need to ensure the struct instance is kept alive, stored on the heap, and freed properly.

Example method function signature for creating a new struct instance:

```rust
pub extern "C" fn create_foo(foo_ptr: *mut *mut Foo) -> i32
```

This function creates a new `Foo` instance and returns its pointer in `foo_ptr`, we can use the same strategies of **Standalone Functions** for getting arguments / setting results.
Example for freeing an instance:

```rust
pub extern "C" fn free_foo(foo_ptr: *const Foo)
```

This function frees the `Foo` instance created previously. Note that functions for freeing instances do not return results and have no arguments other than the pointer to the struct to be freed.
Example for calling Method Functions

```rust
pub extern "C" fn bar_method_foo(foo_ptr: *const Foo)
```

In Rust we use the `foo_ptr` and dereference it to get the instance of `Foo`, we can use the same strategies of **Standalone Functions** for getting arguments / setting results.

## ⚠️ Memory Safety Considerations

- **Memory Allocation**: When working with pointers and heap-allocated structs, be cautious of memory leaks. Ensure that any struct allocated on the heap is explicitly freed after use using the corresponding `free_*` function.

- **Data Integrity**: Ensure that any data passed to Rust (or vice versa) remains valid and is not modified/moved or freed prematurely during function calls.
