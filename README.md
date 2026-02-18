Rewriting Rust's standard library for fun.

## Structs

### collections

#### Vec&lt;T&gt;

Vector with support for slice indexing and implements `.sort()`.

#### SLinkedList&lt;T&gt;

Immutable singly linked list which shares common tails.

### rc

#### Rc&lt;T&gt;

Reference counter, currently only supports strong counters. I will add support for weak counters when I'm writing a collection that needs it.
