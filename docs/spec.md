# Yolk Language Specification

## Table of Contents

- [Syntax](#syntax)
- [Numbers](#numbers)
- [Arrays](#arrays)
- [Variables](#variables)
- [Unary Operators](#unary-operators)
- [Binary Operators](#binary-operators)
- [Built-ins](#built-ins)
- [Functions](#functions)
- [Reserved Keywords](#reserved-keywords)
- [Undefined Behaviour](#undefined-behaviour)

## Syntax

Yolk programs are made up of statements. Statements must be separated by newlines. Any text on a line that follows `//` is considered to be a comment.

## Numbers

A Yolk number is a limited-precision decimal. Yolk numbers have the same precision and range as [Yolol numbers](https://wiki.starbasegame.com/index.php/YOLOL#Decimals).

Numbers may be represented:
- directly, with a literal (e.g. `0`)
- indirectly, with a variable (e.g. `foo`)

For boolean logic, `0` is falsy and all other numbers are truthy. The default truth value is `1`.

## Arrays

A Yolk array is a non-empty sequence of numbers.

Arrays may be represented:
- directly, with a list of numbers (e.g. `[0, 1, foo]`)
- indirectly, with a variable (e.g. `foo`)

Arrays must not be nested.

## Variables

A Yolk variable stores a number or array.

Variable names may contain letters and undescores, and must be unique.

Variables may be assigned to with a `let` statement (e.g. `let foo = 0`). Variables must not be reassigned.

Variables may be imported from Yolol to Yolk with an `import` statement (e.g. `import foo`). Importing a Yolol string will cause [undefined behaviour](#undefined-behaviour). Yolol data fields must not be imported. Variables must not be imported twice.

Variables may be exported from Yolk to Yolol with an `export` statement (e.g. `export foo`). Exported arrays will be split into one Yolol variable per element. Variables must not be exported twice. The names of exported variables must have unique lower-case representations (e.g. cannot export both `foo` and `FOO`).

## Unary Operators

Unary operators perform operations on a single number or array (e.g. `sin(0)`). The operand must be surrounded by parentheses.

Yolk provides the following unary operators:

- `not`: Returns `0` if a number is truthy, or `1` if a number is falsy.
- `abs`: Calculates the absolute value of a number
- `sqrt`: Calculates the square root of a number. Causes [undefined behaviour](#undefined-behaviour) if the number is negative.
- `sin`: Calculates the sine of a number in degrees.
- `cos`: Calculates the cosine of a number in degrees.
- `tan`: Calculates the tangent of a number in degrees. Causes [undefined behaviour](#undefined-behaviour) if the number is outside of the domain.
- `asin`: Calculates the inverse sine of a number in degrees. Causes [undefined behaviour](#undefined-behaviour) if the number is outside of the domain.
- `acos`: Calculates the inverse cosine of a number in degrees. Causes [undefined behaviour](#undefined-behaviour) if the number is outside of the domain.
- `atan`: Calculates the inverse tangent of a number in degrees.

If a unary operator is applied to an array, the operation will be performed element-wise.

```
abs([-1, 0, 1])
// Returns [1, 0, 1]
```

## Binary Operators

Binary operators perform operations on two numbers or arrays (e.g. `1 + 2`).

Yolk provides the following binary operators:

- `+`: Adds two numbers.
- `-`: Subtracts two numbers.
- `*`: Multiplies two numbers.
- `/`: Divides two numbers. Causes [undefined behaviour](#undefined-behaviour) if the divisor is `0`.
- `%`: Computes the modulo of two numbers. Causes [undefined behaviour](#undefined-behaviour) if the divisor is `0`.
- `^`: Raises one number to the power of another. Causes [undefined behaviour](#undefined-behaviour) is `0` is raised to the power of `-1`.
- `<`: Returns true if the first number is less than the second number, otherwise false.
- `<=`: Returns true if the first number is less than or equal to the second number, otherwise false.
- `>`: Returns true if the first number is greater than the second number, otherwise false.
- `>=`: Returns true if the first number is greater than or equal to the second number, otherwise false.
- `==`: Returns true if the numbers are equal, otherwise false.
- `!=`: Returns true if the numbers are not equal, otherwise false.
- `and`: Returns true if both numbers are true, otherwise false.
- `or`: Returns true if either number is true, otherwise false.

If a binary operator is applied to two numbers, the operation will proceed normally.

```
1 + 2
// Returns 3
```

If a binary operator is applied to two arrays, the operation will be performed element-wise. The arrays must have the same length.

```
[1, 2] + [3, 4]
// Returns [4, 6]
```

If a binary operator is applied to a number an array, the operation will be distributed over the array.

```
1 + [2, 3]
// Returns [3, 4]
```

## Built-ins

Built-ins (or builtins) are special functions provided by the transpiler (e.g. `sum(0, [1, 2])`).

Yolk provides the following built-ins:

- `sum`: Add any non-empty sequence of numbers and arrays together to produce a single number.
- `product`: Multiply any non-empty sequence of numbers and arrays together to produce a single numbers.

## Functions

Yolk functions perform operations on values.

Function names may contain letters and underscores, and must be unique.

Functions may be defined with a `define` statement (e.g. `define A(B) = B+1`). Functions must not be redefined.

A `define` statement has 5 parts:

- The `define` keyword
- The name of the function
- The parameters of the functions, surrounded by parentheses. Functions must have at least one parameter.
- An equals sign
- The body of the function

Every function has its own local scope. The body of a function must not access variables outside of its parameter list.

Functions may be called by other code (e.g. `foo(0, 1, [2, 3])`). Functions must not call themselves.

## Reserved Keywords

The following keywords are reserved, and may not be used as variable or function names:

- `import`
- `define`
- `let`
- `export`
- `not`
- `abs`
- `sqrt`
- `sin`
- `cos`
- `tan`
- `asin`
- `acos`
- `atan`
- `and`
- `or`

## Undefined Behaviour

Undefined behaviour occurs when a program performs illegal operations at runtime. Anything may happen!
