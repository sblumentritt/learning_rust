## What is Ownership

- memory is managed through a system of ownership with a set of rules
- **none** of the ownership features **slow down** a program while it's running

## The Stack and the Heap

- the `stack` stores values in the order it gets them
- the `stack` removes the values in the opposite order

-> referred to **last in, first out**
-> adding data is called **pushing onto the stack**
-> removing data is called **popping off the stack**

- all data stored on the `stack` must have a **known**, **fixed** size
- data with an unknown size at compile time or a size that might change must be
  stored on the `heap`

- the `heap` is less organized
  1. request a certain amount of space
  2. operating system finds empty stop in the `heap` that is big enough
  3. marks it as being in use
  4. returns a pointer, which is the address of that location

-> the process is called **allocating on the heap**

- pushing on the `stack` is **faster** than allocating on the `heap`
- accessing data in the `heap` is **slower** than accessing data on the `stack`

## Ownership Rules

- each value has a variable that is called its owner
- there can only be one owner at a time
- when the owner goes out of scope, the value will be dropped

## Rules of References

- at any given time, have either **one mutable** reference or **any number** of
  **immutable** references
- references must always be valid
