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
        "unwrap",
        "expect",
        "unsafe",
    ],
    "Arithmetic Issues": [
        "== is a garbage value",
        "arithmetic",
        "integer overflow",
        "operator precedence",
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
    ],
    "Type Safety": [
        "incompatible integer to pointer conversion",
        "converts between pointers to integer types",
        "passing argument to parameter",
        "incompatible pointer types passing",
    ]
}