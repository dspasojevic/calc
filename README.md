# Command line calculator

A simple command line calculator that can perform basic arithmetic operations.

## Instatllation

Can be installed using cargo:

```bash
cargo install --path .
```

## Usage

The calculator can be used as a REPL or as a command line tool.

From the command line:

```bash
calc "1 + 1 * 100"
```

will output the result of evaluating the expression:

```bash
101 = +
      ├─ 1
      └─ 100 = *
               ├─ 1
               └─ 100
```

Used a REPL:

```bash
calc
```

The REPL uses [reedline](https://github.com/nushell/reedline) for line editing and history.

Additionally, the REPL supports assignment of variables:

```bash
> 〉a:=1 + 300
301 = +
      ├─ 1
      └─ 300
> 〉b:=a * 2
602 = *
      ├─ 301 (a)
      └─ 2
> 〉a + b
903 = +
      ├─ 301 (a)
      └─ 602 (b)
```

Variables can be shown using the `:state` command:

```bash
> 〉:state
a = BinaryOperation { lhs: Integer { value: 1, variable: None }, op: Add, rhs: Integer { value: 300, variable: None }, value: 301.0 }
b = BinaryOperation { lhs: Float { value: 301.0, variable: Some(Variable { name: "a", expr: BinaryOperation { lhs: Integer { value: 1, variable: None }, op: Add, rhs: Integer { value: 300, variable: None }, value: 301.0 } }) }, op: Multiply, rhs: Integer { value: 2, variable: None }, value: 602.0 }
```

Variables can be cleared using the `:clear` command.

Finally, the REPL supports the `:debug` command to show information about the previously evaluated expression:

```bash
> 〉12 * 1.7
20.4 = *
       ├─ 12
       └─ 1.7
> 〉:debug
Some(BinaryOperation { lhs: Integer { value: 12, variable: None }, op: Multiply, rhs: Float { value: 1.7, variable: None }, value: 20.4 })
```
