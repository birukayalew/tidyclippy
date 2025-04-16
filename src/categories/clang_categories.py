CLANG_CATEGORY_MAP = {
    "Undefined Behavior": [
        "undeclared",
        "file not found",
        "implicit function declaration",
        "expected function body",
    ],
    "Security/Insecure APIs": [
        "insecure",
        "strcpy",
    ],
    "Memory Safety": [
        "garbage value",
        "uninitialized",
        "assigned value is garbage",
        "allocation size of 0 bytes",
        "unsafe",
        "should be a buffer with",

    ],
    "Arithmetic Issues": [
        "== is a garbage value",
        "arithmetic",
        "integer overflow",
        "operator precedence",
        "division by zero",
        "right operand is negative",
    ],
    "Dead Code": [
        "unreachable",
        "value stored to",
        "unused",
    ],
    "Redundant Operations": [
        "self-assignment",
        "redundant",
    ],
    "Memory Leaks": [
        "leak",
    ],
    "Null Pointer Dereference": [
        "null pointer",
        "is null",
    ],
    "Type Safety": [
        "type name",
        "incompatible integer to pointer conversion",
        "converts between pointers to integer types",
        "passing argument to parameter",
        "incompatible pointer types passing",
        "no member named",
        "incompatible pointer to integer conversion",
        "implicit conversion from",
        "format specifies type",
        "invalid application of 'sizeof' to an incomplete type",
    ], 
    "Code Smells": [
        "deprecated",
    ],
    "Build Configuration Error": [
        "initializer element is not a compile-time constant",
        "macro redefined",
        "requires inclusion of the header",
        "expected expression",
        "bit-rotted",
        "excess elements in array initializer",
    ],
}