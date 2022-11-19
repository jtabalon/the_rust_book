# Generic Types, Traits, and Lifetimes

---
*This section covers tools which handle the duplication of concepts*
---

The outline is as follows:

1. Extract function to reduce code duplication
2. Make a generic function that takes parameters and run sazme code on multiple concrete values
3. use *traits* to define behavior generically
4. lifetimes: give compiler information about how references relate to each other.

**Generics**
- abstract stand ins for concrete types or other properties

Functions can take parameters of some generic type, instead of a concrete type like `i32` or `String`.

