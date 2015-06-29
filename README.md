# Symbolic Polynomials
[![Build Status](https://travis-ci.org/Botev/symbolic_polynomials.svg?branch=master)](https://travis-ci.org/Botev/symbolic_polynomials) 
[![Crates.io](https://img.shields.io/crates/v/symbolic_polynomials.svg)](https://crates.io/crates/symbolic_polynomials)
[![License](http://img.shields.io/:license-GPLv3+-blue.svg)](https://github.com/Botev/symbolic_polynomials/blob/master/LICENSE)

Symbolic Polynomials is a small package in Rust designed for manipulation of multivraite symbolic polynomials. It was not designed for a single variable polynomials, but can handle it effortlessly. 

## Usage and installation

The package is fully self contained with no dependencies and works with any of the current Rust releases. If you want to use it in your project you just have to add the following line to your `Cargo.toml`:
```rust
[dependencies]
symbolic_polynomials = "*"
```
And to top crate module file:
```rust
extern crate symbolic_polynomials;
```

## Documentation and Example

The API documentation and some basic example usage can be found [here](http://botev.github.io/symbolic_polynomials/). 

**Important** - all of the methods currently are implemented for references to `SymPolynomial`, thus the syntax might be a bit ugly. For an explanation why see the *Discussion* section below.

Just a quick run trough some of the code in the example in the `tests` directory:

1. Import the library and declare usage
```rust
extern crate symbolic_polynomials;
use symbolic_polynomials::SymPolynomial;
```
2. Create a symbolic variable. The syntax indicates that you will be using maximum of 2 symbolic variables, let's call them `a` and `b`. This line declares a `ref` to a first order monomial of the first variable.
```rust
let a = &SymPolynomial::get_first_order(0,2);
```

3. Creates a polynomial representing the sum `a+b`
```rust
let a_plus_b = &(a + b);
```
4. This is where code can get a bit ugly, this represents `a_square + b_square + 2*a_b`:
```rust
let a_plus_b_square = &(&(a_square + b_square) + &(2*a_b));
```
5. This represents `a^3-1`
```rust
let a_3 = &(&(a * a) * a) - 1;
```
6. Division of polynomials returns an `Option<SymPolynomial>`. It is `None` if they are not divisible. Thus this returns `Some`:
```rust
let a_res = &a_3 / &a_minus_1;
```
while this returns a `None`
```rust
&a_3_plus_2 / &a_minus_1
```

## Discussion and Caveats
The library has implemented all methods for references for one reason only - the `SymPolynomial` uses a  `Vec<SymMonomial>` as the internal representation, therefore it is not copyable. This means that implementing for objects would consume the variables, which is much more undesirable. Also, the library will `panic` on most methods if it is called with polynomials with different number of symbolic variables. This is for a similar reason - currently Rust does not support a `int` tempalte argumetns, thus the `SymMonomial` also uses as an internal representation a `Vec`. 

At this point I have not sit down to actually optimise all of the methods, thus I would not consider them optimised. What is more if you need a single variable polynomials, it is highly possible that there is much better representation and implementations out there.

