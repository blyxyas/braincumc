# Compiler Spec

- [Compiler Spec](#compiler-spec)
	- [General](#general)

## General
The general way is to have a vector, which may contain unsigned 8 bit integers (up to 255) which may be displayed as characters, or other vectors (scopes)

Each scope has its own cells, and while the global scope is infinite, sliced array scopes are joined as a loop.