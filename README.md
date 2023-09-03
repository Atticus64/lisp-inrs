# Interpreter of Lisp language

> For learning purposes

## Features ☕

* Various Data Types
    * String `"Jona"` [X]
    * Integer `10` [X]
    * Float `3.1416` [X]
    * Lambda `(lambda (x) (+ x 1))` [X]
    * Boolean `true` [X]

* Built-in Functions
    * `+` Add [X]
    * `-` Subtract [X]
    * `*` Multiply [X]
    * `/` Divide [X]
    * `^` Pow [X]
    * `define` For define variables and functions [X]
    * `load` For loading files [X]
    * `print` For Debugging [X]


### Examples

* Add two numbers

```lisp
(define add (lambda (x y) (+ x y)))
(add 5)
```

* Circle area

```lisp 
(define pi 3.1416)
(define circle-area (lambda (r) (* pi (* r r))))
(circle-area 5)
```

* Print Hello World

```lisp
(print "Hello World")
```

