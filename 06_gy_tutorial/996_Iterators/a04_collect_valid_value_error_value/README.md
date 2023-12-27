# Result

- https://doc.rust-lang.org/rust-by-example/error/iter_result.html  
```
Numbers: [42, 93, 999, 18]
Errors: [ParseIntError { kind: InvalidDigit }]
```

# fn partition

https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.partition

- Consumes an iterator, creating two collections from it.

  - The predicate passed to partition() can return true, or false. partition() returns a pair, all of the elements for which it returned true, and all of the elements for which it returned false.
