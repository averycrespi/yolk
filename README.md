<h1 align="center">
    <br>
    <img src="https://raw.githubusercontent.com/averycrespi/yolk/master/resources/yolk.png" width="150"</img>
    <br>
    Yolk
    <br>
</h1>

<h4 align="center">Numerical computing for YOLOL.</h4>

<p align="center">
    <a href="#what-is-yolk">What</a> •
    <a href="#why-should-i-use-yolk">Why</a> •
    <a href="#how-do-i-get-started">How</a> •
    <a href="#license">License</a> •
    <a href="#credits">Credits</a>
</p>

**Warning: Yolk is currently in development. Things may break at any time!**

## What is Yolk?

Yolk is a [domain-specific language](https://en.wikipedia.org/wiki/Domain-specific_language) that transpiles to [YOLOL](https://wiki.starbasegame.com/index.php/YOLOL).

## Why should I use Yolk?

#### Convenience

YOLOL doesn't support arrays, so each element must be a separate variable.

```
// YOLOL
a_0=1 a_1=2 a_2=4 a_3=8
```

Yolk handles array expansion so that you don't have to.

```
// Yolk
let a = [1, 2, 4, 8];
```

In YOLOL, applying element-wise operations to an array is repetitive and error-prone.

```
// YOLOL
a_0=1 a_1=2 a_2=4 a_3=8
b_0=a_0^2 b_1=a_1^2 b_2=a_2^2 b_3=a_3^2
// Result: b_0 == 1, b_1 == 4, b_2 == 16, b_3 == 64
```

In Yolk, element-wise operations are powerful and concise.

```
// Yolk
let a = [1, 2, 4, 8];
let b = a ^ 2;
// Result: b == [1, 4, 16, 64]
```

#### Simplicity

Yolk's syntax is simple and beginner-friendly.

```
// This is a comment!
import a;
define b(c, d) = c + d;
let e = 0;
export f;
```

Yolk provides the same operators and precedence as YOLOL.

```
let number = 1 + 2 * 3;
// Result: number == 7
```

#### Efficiency

YOLOL chips are slow and have a limited amount of space.

Yolk aggressively optimizes your code to make it faster and smaller.

TODO

## How do I get started?

TODO

## License

[MIT](https://opensource.org/licenses/MIT)

## Credits

Logo derived from: Egg by David from the [Noun Project](https://thenounproject.com/)
