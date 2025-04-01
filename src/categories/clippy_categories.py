CLIPPY_CATEGORY_MAP = {
    "Dead Code": [
        "unused",
        "unreachable",
        "is never read",
    ],
    "Undefined Behavior": [
        "panic",
        "undeclared",
    ],
    "Memory Safety": [
        "unwrap",
        "unsafe",
        "manual memory management",
    ],
    "Null Pointer Dereference": [
        "null",
    ],
    "Type Safety": [
        "casting",
        "borrowed",
        "reference",
        "extern types are experimental",
        "lossy conversion",
    ],
    "Memory Leaks": [
        "leak",
    ],
    "Arithmetic Issues": [
        "integer overflow",
        "operator precedence",
    ],
    "Redundant Operations": [
        "redundant",
    ],
    "Security/Insecure APIs": [
        "security",
    ],
    "Code Smells": [
        "crate-level attribute",
        "can be collapsed",
        "unnecessary double parentheses",
        "unnecessary parentheses",
        "branch is empty",
        "should have an upper camel case name",
        "variable does not need to be mutable",
    ],
    "Build Configuration Error": [
        "#![feature]",
        "function not found in crate",
        "has been stable",
        "inner attribute",
        "unresolved import",
        "cannot find function",
        "c-variadic functions are unstable",
        "use of unstable library feature",
    ],
}
