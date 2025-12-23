<p align="center">
    <strong>English</strong>
  <a href="README_RU.md">Русский</a> ·
</p>

# The Lazy Programming Language v1.1

**The world's first programming language with ZERO keywords!**

## What's New in v1.1

- **Smart Input System** - Custom prompts, multiple variables, iterative inputs with `+?`
- **If-Else-If Chains** - Full conditional logic with `??` for else-if and else
- **Quick Functions** - One-liner function definitions with `~>`
- **Enhanced List Operations** - Reverse, sort, search, unique operations
- **Full Unicode Support** - Variables in ANY language (русский, 中文, العربية, etc.)
- **Arrow Parameters** - Use `->` in function calls for clarity
- **Inline Input** - Use `+??` for temporary input in expressions
- **Mutation Operators** - Use `*` suffix to mutate lists in-place
- **Scoped Blocks** - Use `{}` for clear code structure

## What is Lazy?

Lazy is a revolutionary programming language that uses **only symbols** - no keywords at all! If you can understand arrows and symbols, you can code in Lazy.

## Installation & Setup

### Building the Interpreter

1. **Create the project:**
```bash
cargo new lazy
cd lazy
```

2. **Replace `src/main.rs`** with the Lazy interpreter code

3. **Build the executable:**
```bash
cargo build --release
```

4. **The executable will be at:**
- Windows: `target\release\lazy.exe`
- Linux/Mac: `target/release/lazy`

### Running Lazy Programs

**Option 1: Run a .lazy file**
```bash
# Windows
.\target\release\lazy.exe program.lazy

# Linux/Mac
./target/release/lazy program.lazy
```

**Option 2: Interactive REPL**
```bash
# Windows
.\target\release\lazy.exe

# Linux/Mac
./target/release/lazy
```

Type code, then type `run` to execute it.

## The Symbol System

Lazy has **ZERO keywords**. Everything is done with symbols:

| Symbol | Purpose                      | Memory Aid                     |
|--------|------------------------------|--------------------------------|
| `+?`   | Input with prompt            | Plus question = asking for input |
| `+??`  | Inline input                 | Double question = temporary input |
| `?`                               | If OR Else-if OR else        | Double question = alternative |
| `@`    | Loop                         | Looks like circular motion     |
| `->`   | Return OR arrow to parameter | Arrow pointing forward         |
| `=>`   | Start function               | Fat arrow = beginning          |
| `~>`   | Quick function               | Wavy arrow = shortcut          |
| `}`    | End any block                | Closing brace = ending         |
| `>>`   | For-each loop                | Fast forward through items     |
| `=`    | Assign variable              | Equals = store                 |
| `*`    | Mutate in-place              | Star = modify original         |

## Built-in Symbol Functions

Instead of named functions like `len()`, Lazy uses **symbol operators**:

### Core Operations
| Symbol        | Function | Example                           |
|---------------|----------|-----------------------------------|
| `#(x)`        | Length/count | `#(mylist)` → 5                   |
| `$(x)`        | To string | `$(42)` → "42"                    |
| `~(x)`        | To number | `~("42")` → 42                    |
| `!(bool)`     | Not/opposite | `!(yes)` → no                     |
| `?=(max)`     | Random 0 to max | `?=(10)` → random 0-9             |

### List Manipulation
| Symbol        | Function | Example                           |
|---------------|----------|-----------------------------------|
| `^(list -> val)` | Push (add to end) | `^([1 2] -> 3)` → [1 2 3]            |
| `v(list)`     | Pop (remove last) | `v([1 2 3])` → [1 2]              |
| `<>(list)`    | Reverse | `<>([1 2 3])` → [3 2 1]           |
| `++(list)`    | Sort ascending | `++([3 1 2])` → [1 2 3]           |
| `--(list)`    | Sort descending | `--([1 3 2])` → [3 2 1]           |
| `><(list -> val)` | Contains/search | `><([1 2 3] -> 2)` → yes             |
| `<<(list)`    | Remove duplicates | `<<([1 2 2 3])` → [1 2 3]         |

### String Operations
| Symbol        | Function | Example                           |
|---------------|----------|-----------------------------------|
| `&(list -> sep)` | Join to string | `&([1 2 3] -> "-")` → "1-2-3"     |
| `\|(str -> sep)` | Split to list | `\|("a-b-c" -> "-")` → ["a" "b" "c"] |

### Mutation Operators

By default, list operations return **new lists** without modifying the original:

```lazy
numbers = [1 2 3]
new_numbers = ^(numbers -> 4)   // Returns new list [1 2 3 4]
numbers                          // Still [1 2 3]
```

**Use `*` suffix to mutate the original list:**

```lazy
numbers = [1 2 3]
^(numbers -> 4)*                 // Mutates numbers to [1 2 3 4]
numbers                          // Now [1 2 3 4]
```

**Works with all list operations:**

```lazy
list = [3 1 4 1 5]

++(list)*                        // Sort in place
--(list)*                        // Sort descending in place
<>(list)*                        // Reverse in place
<<(list)*                        // Remove duplicates in place
v(list)*                         // Pop in place
```

**Why this design?**
- **Safe by default** - Operations don't accidentally modify your data
- **Explicit mutations** - The `*` makes it clear when you're changing the original
- **Flexible** - Choose whether to keep the original or modify it

## Core Syntax

### 1. Printing - Just Write It

```lazy
"Hello World"
42
"The answer"
```

No print keyword needed!

### 2. Variables (Unicode Support!)

```lazy
x = 5
name = "Bob"
имя = "Марат"
名前 = "太郎"
items = [1 2 3]
```

**Lazy supports ANY unicode characters in variable names!** Use your native language!

### 3. Math Operations

```lazy
x = 10 + 5
y = 20 - 3
z = 4 * 5
result = 100 / 2
remainder = 10 % 3
```

Operators: `+` `-` `*` `/` `%` (modulo)

### 4. Smart Input System

**Basic input:**
```lazy
+? name
"Hello " + name
```

**With custom prompt:**
```lazy
+? name : "What's your name? "
"Hello " + name
```

**Multiple variables at once:**
```lazy
+? x y z : "Enter 3 numbers: "
"You entered: " + $(x) + ", " + $(y) + ", " + $(z)
```

**Iterative input (numbered prompts):**
```lazy
+? num1 num2 num3 : "Enter number {?}: "
```
This will show:
```
Enter number 1: 
Enter number 2: 
Enter number 3: 
```

**Inline temporary input with `+??`:**
```lazy
add(a b c) ~> a + b + c

result = add(5 +?? +??)
"Result: " + $(result)
```

This allows you to input values directly in function calls or expressions without creating variables!

**Multiple inline inputs:**
```lazy
multiply(a b c) ~> a * b * c

// All three values from input
multiply(+?? +?? +??)

// Mix variables and input
multiply(10 +?? 5)
```

### 5. Functions

**Standard functions:**
```lazy
greet(name) => {
  "Hello " + name
}

greet("Alice")
```

**Quick functions with `~>`:**
```lazy
add(a b) ~> a + b
square(x) ~> x * x
double(n) ~> n * 2

add(5 3)
square(4)
double(10)
```

One line, no braces needed!

**With parameters:**
```lazy
max(a b) => {
  ? a > b {
    -> a
  }
  -> b
}

bigger = max(10 20)
bigger
```

**Using arrow parameters for clarity:**
```lazy
&(["apple" "banana" "cherry"] -> ", ")
|(text -> " ")
^(list -> 42)
><(items -> 5)
```

### 6. If-Else-If Chains

```lazy
score = 85

? score >= 90 {
  "Grade: A"
}
?? score >= 80 {
  "Grade: B"
}
?? score >= 70 {
  "Grade: C"
}
?? {
  "Grade: F"
}
```

- `?` for if
- `??` for else-if (with condition)
- `??` for else (without condition)
- `{}` defines scope

**Simple if:**
```lazy
? x > 5 {
  "x is big"
}
```

Comparison operators: `>` `<` `==` `!=` `>=` `<=`

### 7. Loops - `@` Symbol

```lazy
counter = 0

@ counter < 5 {
  counter
  counter = counter + 1
}

"Done"
```

### 8. For-Each Loop - `>>` Symbol

```lazy
items = [1 2 3 4 5]

>> item items {
  item * 2
}
```

With unicode variables:
```lazy
числа = [1 2 3 4 5]

>> число числа {
  число * 2
}
```

### 9. Lists (Arrays)

**Create lists:**
```lazy
numbers = [1 2 3 4 5]
names = ["Alice" "Bob" "Charlie"]
mixed = [1 "hello" 3]
empty = []
```

**Access items (0-indexed, supports negative indices):**
```lazy
numbers = [10 20 30]
first = numbers[0]      // 10
last = numbers[-1]      // 30 (negative = from end)
second = numbers[1]     // 20
```

**Basic operations:**
```lazy
items = [1 2 3 4 5]

// Length
#(items)             // 5

// Push (returns new list)
new_list = ^(items -> 6)        // [1 2 3 4 5 6]
items                            // Still [1 2 3 4 5]

// Push (mutates original)
^(items -> 6)*                   // Mutates items
items                            // Now [1 2 3 4 5 6]

// Pop (remove last)
v(items)             // [1 2 3 4]

// Combine
list1 + list2        // Concatenate lists
```

**Advanced List Operations:**
```lazy
numbers = [3 1 4 1 5 9 2 6]

// Reverse (returns new)
reversed = <>(numbers)          // [6 2 9 5 1 4 1 3]
numbers                          // Still [3 1 4 1 5 9 2 6]

// Reverse (mutates)
<>(numbers)*                     // Mutates numbers
numbers                          // Now [6 2 9 5 1 4 1 3]

// Sort ascending
sorted_asc = ++(numbers)         // [1 1 2 3 4 5 6 9]

// Sort descending (mutates)
--(numbers)*                     // Sorts in place

// Search/contains
><(numbers -> 5)                 // yes

// Remove duplicates
unique = <<(numbers)             // [3 1 4 5 9 2 6]
```

**Real example with mutations:**
```lazy
scores = [85 92 78 85 90]

// Remove duplicates in place
<<(scores)*

// Sort them in place
++(scores)*

scores                           // [78 85 90 92]

// Check if 90 exists
has_90 = ><(scores -> 90)
has_90                           // yes
```

### 10. String Operations

**Concatenate:**
```lazy
first = "Hello"
second = "World"
message = first + " " + second
```

**Convert number to string:**
```lazy
age = 25
text = "I am " + $(age) + " years old"
```

**Convert string to number:**
```lazy
input = "42"
num = ~(input)
result = num + 10
```

**Join list to string (with arrow):**
```lazy
words = ["Hello" "World" "Lazy"]
sentence = &(words -> " ")
sentence                          // "Hello World Lazy"

// Or with comma
csv = &(words -> ", ")            // "Hello, World, Lazy"
```

**Split string to list (with arrow):**
```lazy
text = "apple,banana,cherry"
fruits = |(text -> ",")
fruits                            // ["apple" "banana" "cherry"]

// Split by space
words = |(sentence -> " ")
```

### 11. Boolean Values

Use `yes` and `no` instead of true/false:
```lazy
is_ready = yes
is_done = no

? is_ready == yes {
  "Let's go"
}
```

**Not operator:**
```lazy
ready = yes
not_ready = !(ready)
not_ready                         // no
```

### 12. Random Numbers

```lazy
dice = ?=(6) + 1                  // Random 1-6
"You rolled: " + $(dice)

coin = ?=(2)                      // Random 0-1
? coin == 0 {
  "Heads"
}
?? {
  "Tails"
}
```

## Complete Examples

### Example 1: Hello User (New Input System)

```lazy
+? name : "What's your name? "
+? age : "How old are you? "

"Hello " + name + "!"
"You are " + $(age) + " years old."
```

### Example 2: Multiple Inputs

```lazy
+? x y z : "Enter value: "

sum = x + y + z
"Sum: " + $(sum)

average = sum / 3
"Average: " + $(average)
```

### Example 3: Iterative Input

```lazy
+? num1 num2 num3 : "Enter number {?}: "

total = num1 + num2 + num3
"Total: " + $(total)
```

### Example 4: Inline Input

```lazy
add(a b c) ~> a + b + c
multiply(a b) ~> a * b

"Calculator with inline input"

"Addition:"
result1 = add(+?? +?? +??)
"Result: " + $(result1)

"Multiplication:"
result2 = multiply(+?? +??)
"Result: " + $(result2)
```

### Example 5: Grade Calculator

```lazy
+? score : "Enter your score: "

? score >= 90 {
  "Grade: A - Excellent!"
}
?? score >= 80 {
  "Grade: B - Good job!"
}
?? score >= 70 {
  "Grade: C - Not bad"
}
?? score >= 60 {
  "Grade: D - Need improvement"
}
?? {
  "Grade: F - Study more!"
}
```

### Example 6: Quick Math Functions

```lazy
// Define quick functions
add(a b) ~> a + b
subtract(a b) ~> a - b
multiply(a b) ~> a * b
divide(a b) ~> a / b

+? x y : "Enter two numbers: "

"Sum: " + $(add(x y))
"Difference: " + $(subtract(x y))
"Product: " + $(multiply(x y))
"Quotient: " + $(divide(x y))
```

### Example 7: List Sorting with Mutations

```lazy
numbers = [42 17 8 99 23 56 31]

"Original: "
numbers

"Sorted ascending (new list): "
++(numbers)

"Original still unchanged: "
numbers

"Now sorting in place:"
++(numbers)*
numbers

"Reversing in place:"
<>(numbers)*
numbers
```

### Example 8: Remove Duplicates

```lazy
items = [1 2 3 2 4 3 5 1]

"Original: "
items

"Removing duplicates in place:"
<<(items)*
items

"Sorting in place:"
++(items)*
items
```

### Example 9: Text Processing

```lazy
+? text : "Enter words separated by spaces: "

words = |(text -> " ")
"You entered " + $(#(words)) + " words"

"Words: "
>> word words {
  "- " + word
}

"Joined with commas: "
&(words -> ", ")
```

### Example 10: Shopping List with Mutations

```lazy
items = []

add_item() => {
  +? item : "Add item: "
  ^(items -> item)*              // Mutate items directly
}

add_item()
add_item()
add_item()

"Your list:"
>> item items {
  item
}

"Total: " + $(#(items))
```

### Example 11: Number Guessing Game

```lazy
secret = ?=(10) + 1

"I'm thinking of a number 1-10"

+? guess : "Your guess: "

? guess == secret {
  "You win!"
}
?? guess > secret {
  "Too high!"
}
?? {
  "Too low!"
}
```

### Example 12: Temperature Converter

```lazy
c2f(celsius) ~> celsius * 9 / 5 + 32
f2c(fahrenheit) ~> (fahrenheit - 32) * 5 / 9

+? temp : "Enter Celsius: "
"Fahrenheit: " + $(c2f(temp))

+? temp : "Enter Fahrenheit: "
"Celsius: " + $(f2c(temp))
```

### Example 13: FizzBuzz

```lazy
fizzbuzz(max) => {
  counter = 1
  @ counter <= max {
    output = ""
    
    ? counter % 3 == 0 {
      output = "Fizz"
    }
    
    ? counter % 5 == 0 {
      output = output + "Buzz"
    }
    
    ? #(output) == 0 {
      $(counter)
    }
    
    ? #(output) > 0 {
      output
    }
    
    counter = counter + 1
  }
}

fizzbuzz(20)
```

### Example 14: Interactive Calculator

```lazy
calculator() => {
  +? operation : "Choose operation (+, -, *, /): "
  
  "Enter two numbers:"
  result = 0
  
  ? operation == "+" {
    result = add(+?? +??)
  }
  ?? operation == "-" {
    result = subtract(+?? +??)
  }
  ?? operation == "*" {
    result = multiply(+?? +??)
  }
  ?? operation == "/" {
    result = divide(+?? +??)
  }
  
  -> result
}

add(a b) ~> a + b
subtract(a b) ~> a - b
multiply(a b) ~> a * b
divide(a b) ~> a / b

result = calculator()
"Result: " + $(result)
```

### Example 15: Unicode Variables

```lazy
// Russian
привет(имя) ~> "Привет, " + имя + "!"
привет("Марат")

// Chinese
加(甲 乙) ~> 甲 + 乙
加(5 10)

// Arabic
رقم = 42
رقم * 2
```

### Example 16: Leaderboard System

```lazy
scores = [450 320 580 420 580]

"Original scores:"
scores

"Removing duplicates in place:"
<<(scores)*
scores

"Sorting descending in place:"
--(scores)*
scores

"Top 3:"
"1st place: " + $(scores[0])
"2nd place: " + $(scores[1])
"3rd place: " + $(scores[2])
```

## Complete Quick Reference

### Input System
```
+?              Input (basic prompt)
+? var          Input single variable
+? a b c        Input multiple variables
+? var : "text" Input with custom prompt
+? a b : "{?}"  Iterative input (numbered)
+??             Inline temporary input
```

### Symbols
```
??     Else-if OR else
@      Loop while condition true
>>     For-each loop through list
->     Return value OR parameter arrow
=>     Start function definition
~>     Quick function (one-liner)
}      End block
=      Assign to variable
*      Mutate in-place (suffix)
```

### Operators
```
// Core
#(x)         Length/count
$(x)         Convert to string  
~(x)         Convert to number
!(bool)      Boolean NOT
?=(max)      Random 0 to max-1

// List basics
^(list -> val)    Push to list
v(list)           Pop from list

// List advanced
<>(list)          Reverse list
++(list)          Sort ascending
--(list)          Sort descending
><(list -> val)   Contains/search
<<(list)          Remove duplicates

// Mutations (add * suffix)
^(list -> val)*   Push and mutate
++(list)*         Sort ascending and mutate
--(list)*         Sort descending and mutate
<>(list)*         Reverse and mutate
<<(list)*         Remove duplicates and mutate
v(list)*          Pop and mutate

// String
&(list -> sep)    Join with separator
|(str -> sep)     Split by separator
```

### Comparisons & Math
```
// Comparisons
>   Greater than
<   Less than
==  Equal to
!=  Not equal
>=  Greater or equal
<=  Less or equal

// Math
+   Add
-   Subtract
*   Multiply
/   Divide
%   Modulo (remainder)
```

### Values
```
yes / no      Booleans
"text"        Strings  
42            Numbers
[1 2 3]       Lists
```

## Mutation vs Immutability

**Default: Immutable (Safe)**
```lazy
numbers = [1 2 3]
new_numbers = ^(numbers -> 4)    // Returns new list
numbers                           // Still [1 2 3]
new_numbers                       // [1 2 3 4]
```

**With `*`: Mutable (Efficient)**
```lazy
numbers = [1 2 3]
^(numbers -> 4)*                  // Mutates numbers
numbers                           // Now [1 2 3 4]
```

**When to use each:**

**Immutable (no `*`)** - Use when:
- You need to keep the original data
- Working with multiple versions of data
- Building data transformation pipelines
- Safety is more important than memory

**Mutable (with `*`)** - Use when:
- You don't need the original data
- Working with large lists (saves memory)
- Building in-place algorithms
- Performance is important

## If-Else Reference

```lazy
// If
? condition {
  // code
}

// If-else
? condition {
  // code
}
?? {
  // else code
}

// If-else-if-else
? condition1 {
  // code
}
?? condition2 {
  // code
}
?? condition3 {
  // code
}
?? {
  // else code
}
```

## Function Reference

```lazy
// Standard function
name(params) => {
  // code
  -> value
}

// Quick function (one-liner)
name(params) ~> expression

// Examples
add(a b) ~> a + b
greet(name) ~> "Hello " + name
square(x) ~> x * x

// With inline input
add(5 +??)              // One input value
add(+?? +??)            // Two input values
multiply(10 +?? 20)     // Mix values and input
```

## Tips & Best Practices

1. **Always use `{}` for blocks** - Makes code structure clear

2. **Use descriptive variable names** - Even in your native language

3. **Space-separate list items** - `[1 2 3]` not `[1,2,3]`

4. **Use `->` for clarity** - `^(list -> value)` is clearer than `^(list value)`

5. **Quick functions for simple logic** - Use `~>` when function is one expression

6. **Leverage mutations for efficiency** - Use `*` when you don't need the original

7. **Custom prompts improve UX** - Always provide clear prompts for input

8. **Use iterative input for similar data** - Great for arrays of numbers

9. **Unicode variables are powerful** - Code in your native language

10. **Negative indices for lists** - `list[-1]` gets last item

11. **Inline input for flexibility** - Use `+??` when you don't need to store the value

12. **Default to immutable** - Use mutations (`*`) only when needed

## Unicode Support

Lazy fully supports Unicode! You can write code in ANY language:

**Russian:**
```lazy
функция(аргумент) ~> аргумент * 2
число = 42
функция(число)
```

**Chinese:**
```lazy
函数(参数) ~> 参数 + 10
数字 = 5
函数(数字)
```

**Arabic:**
```lazy
وظيفة(قيمة) ~> قيمة * 3
رقم = 7
وظيفة(رقم)
```

**Japanese:**
```lazy
関数(値) ~> 値 / 2
数 = 100
関数(数)
```

**Mix languages:**
```lazy
calculate(число value số) ~> число + value + số
calculate(10 20 30)
```

## Philosophy

**Why no keywords?**

Keywords require you to remember specific words in English. Symbols are:
- **Universal** - Work across all languages
- **Visual** - You can see what they do
- **Minimal** - Only 8 core symbols to learn
- **Intuitive** - `@` looks like a loop, `+?` asks for input
- **Inclusive** - Unicode support means code in any language

**Lazy is designed for:**
- Teaching programming basics
- Quick prototyping
- Fun coding experiments
- Educational projects
- Multilingual programmers

## What's Not in Lazy?

Lazy deliberately excludes:
- Classes/Objects
- Imports/Modules
- Exceptions
- Pointers
- Type declarations

This keeps it **lazy** and **easy**!

## Error Handling

Lazy tries to be forgiving:
- Missing `}` - Your block won't close properly
- Wrong type - Operations return `nothing`
- Bad index - Returns `nothing`
- Missing file - Prints error message

## Limitations (By Design)

- **Simple structure** - Keep it straightforward
- **No file I/O** - Just stdin/stdout
- **No classes** - Just functions and data
- **Single file programs** - No imports
- **Dynamic typing only** - No type declarations

These are features, not bugs! Lazy stays lazy!

## Summary of v1.1 Improvements

### Input System
- Custom prompts with `:`
- Multiple variables in one statement
- Iterative input with `{?}`
- Inline input with `+??`

### Control Flow
- If-else-if chains with `??`
- Clear scope with `{}`
- Better readability

### Functions
- Quick functions with `~>`
- Arrow parameters with `->`
- One-liner definitions
- Inline input support

### Lists
- Reverse `<>`
- Sort `++` / `--`
- Search `><`
- Unique `<<`
- Negative indices
- **Mutation operators with `*`**

### Language
- Full Unicode support
- Any language for variables
- International friendly

## Learning Path

1. **Start Simple** - Variables, printing, basic math
2. **Input & Output** - Learn the input system with `+?`
3. **Conditions** - If-else-if chains
4. **Loops** - `@` and `>>`
5. **Functions** - Start with `~>`, move to `=>`
6. **Lists** - Basic operations first
7. **Advanced Lists** - Sorting, searching, mutations
8. **Put It Together** - Build complete programs

## Project Ideas

1. **Calculator** - Use quick functions and inline input
2. **Quiz Game** - Practice if-else chains
3. **Todo List** - Master list mutations
4. **Gradebook** - Sorting and statistics
5. **Text Analyzer** - String operations
6. **Number Guessing** - Random and input
7. **Shopping Cart** - Full CRUD operations
8. **Leaderboard** - Sorting and unique values
9. **Multilingual App** - Use Unicode variables
10. **Data Processor** - All list operations

## Support

Need help? Remember:
- Read error messages carefully
- Check your `}` closing braces
- Use REPL to test snippets
- Start simple, build up complexity
- Remember: `*` for mutations, without for new copies

**Remember:** If it feels like work, you're doing it wrong! Stay lazy!

---

**Version:** 1.1.0 
**License:** Feel free to use and modify  
**Philosophy:** Programming should be easy, visual, and universal