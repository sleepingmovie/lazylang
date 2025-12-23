# The Lazy Programming Language

**The world's first programming language with ZERO keywords!**

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

| Symbol | Purpose | Memory Aid |
|--------|---------|------------|
| `?` | Input OR Condition | Question mark = asking |
| `@` | Loop | Looks like circular motion |
| `->` | Return from function | Arrow pointing forward |
| `=>` | Start function | Fat arrow = beginning |
| `<=` | End any block | Arrow pointing back = ending |
| `>>` | For-each loop | Fast forward through items |
| `=` | Assign variable | Equals = store |

## Built-in Symbol Functions

Instead of named functions like `len()`, Lazy uses **symbol operators**:

| Symbol        | Function | Example                           |
|---------------|----------|-----------------------------------|
| `#(x)`        | Length/count | `#(mylist)` → 5                   |
| `$(x)`        | To string | `$(42)` → "42"                    |
| `~(x)`        | To number | `~("42")` → 42                    |
| `^(list val)` | Push (add to list) | `^([1 2] 3)` → [1 2 3]            |
| `v(list)`     | Pop (remove last) | `v([1 2 3])` → [1 2]              |
| `&(list sep)` | Join to string | `&([1 2 3] "-")` → "1-2-3"        |
| `\|(str sep)` | Split to list | `\|("a-b-c" "-")` → ["a" "b" "c"] |
| `!(bool)`     | Not/opposite | `!(yes)` → no                     |
| `?=(max)`     | Random 0 to max | `?=(10)` → random 0-9             |

**Why symbols?** They're universal and visual!
- `#` looks like counting tally marks
- `$` looks like S for String
- `~` is wavy like numbers
- `^` points up = add/push
- `v` points down = remove/pop
- `&` connects = join
- `|` divides = split
- `!` = opposite/not

## Core Syntax

### 1. Printing - Just Write It

```lazy
"Hello World"
42
"The answer"
```

No print keyword needed!

### 2. Variables

```lazy
x = 5
name = "Bob"
items = [1 2 3]
```

### 3. Math Operations

```lazy
x = 10 + 5
y = 20 - 3
z = 4 * 5
result = 100 / 2
remainder = 10 % 3
```

Operators: `+` `-` `*` `/` `%` (modulo)

### 4. User Input - The `?` Symbol

```lazy
"What's your name?"
? name
"Hello " + name
```

Automatically detects if input is a number or text!

```lazy
"Enter age:"
? age
age
```

### 5. Functions - `=>` and `<=`

**No `function`, `def`, or `fn` keyword!**

```lazy
greet() =>
  "Hello from function"
<=

greet()
```

**With parameters:**
```lazy
add(a b) =>
  -> a + b
<=

result = add(5 3)
result
```

**With logic:**
```lazy
max(a b) =>
  ? a > b
    -> a
  <=
  -> b
<=

bigger = max(10 20)
bigger
```

### 6. Conditionals - `?` Symbol

```lazy
x = 10

? x > 5
  "x is big"
<=

? x == 10
  "x is exactly 10"
<=
```

Comparison operators: `>` `<` `==` `!=` `>=` `<=`

**Note:** Context determines if `?` is input or condition:
- `? name` = input (just variable)
- `? x > 5` = condition (has comparison)

### 7. Loops - `@` Symbol

```lazy
counter = 0

@ counter < 5
  counter
  counter = counter + 1
<=

"Done"
```

### 8. For-Each Loop - `>>` Symbol

```lazy
items = [1 2 3 4 5]

>> item items
  item
<=
```

This iterates through each item in the list!

```lazy
names = ["Alice" "Bob" "Charlie"]

>> name names
  "Hello " + name
<=
```

### 9. Lists (Arrays)

**Create lists with spaces, not commas:**
```lazy
numbers = [1 2 3 4 5]
names = ["Alice" "Bob" "Charlie"]
mixed = [1 "hello" 3]
empty = []
```

**Access items (0-indexed):**
```lazy
numbers = [10 20 30]
first = numbers[0]
first

second = numbers[1]
second
```

**Get length:**
```lazy
items = [1 2 3 4 5]
count = #(items)
count
```

**Add items (push):**
```lazy
numbers = [1 2 3]
bigger = ^(numbers 4)
bigger
```

**Remove last item (pop):**
```lazy
numbers = [1 2 3 4]
smaller = v(numbers)
smaller
```

**Combine lists:**
```lazy
list1 = [1 2 3]
list2 = [4 5 6]
combined = list1 + list2
combined
```

### 10. String Operations

**Concatenate:**
```lazy
first = "Hello"
second = "World"
message = first + " " + second
message
```

**Convert number to string:**
```lazy
age = 25
text = "I am " + $(age) + " years old"
text
```

**Convert string to number:**
```lazy
input = "42"
num = ~(input)
result = num + 10
result
```

**Join list to string:**
```lazy
words = ["Hello" "World" "Lazy"]
sentence = &(words " ")
sentence
```

**Split string to list:**
```lazy
text = "apple,banana,cherry"
fruits = |(text ",")
fruits
```

### 11. Boolean Values

Use `yes` and `no` instead of true/false:
```lazy
is_ready = yes
is_done = no

? is_ready == yes
  "Let's go"
<=
```

**Not operator:**
```lazy
ready = yes
not_ready = !(ready)
not_ready
```

### 12. Random Numbers

```lazy
dice = ?=(6)
"You rolled:"
dice
```

Generates random number from 0 to max-1.

## Complete Examples

### Example 1: Hello User

```lazy
"What's your name?"
? name
"Hello " + name + "! Welcome to Lazy!"
```

### Example 2: Calculator

```lazy
"Enter first number:"
? a
"Enter second number:"
? b

sum = a + b
"Sum: " + $(sum)

product = a * b
"Product: " + $(product)
```

### Example 3: Countdown

```lazy
count = 10

@ count > 0
  count
  count = count - 1
<=

"Liftoff!"
```

### Example 4: Guessing Game

```lazy
secret = 7

"Guess a number 1-10:"
? guess

? guess == secret
  "You win!"
<=

? guess > secret
  "Too high!"
<=

? guess < secret
  "Too low!"
<=
```

### Example 5: Factorial Function

```lazy
factorial(n) =>
  ? n <= 1
    -> 1
  <=
  result = n * factorial(n - 1)
  -> result
<=

"Enter a number:"
? num
answer = factorial(num)
$(num) + " factorial is " + $(answer)
```

### Example 6: List Operations

```lazy
numbers = [1 2 3 4 5]
"Original list:"
numbers

"List length:"
#(numbers)

"Add 6:"
numbers = ^(numbers 6)
numbers

"Remove last:"
numbers = v(numbers)
numbers

"First item:"
numbers[0]
```

### Example 7: For-Each Loop

```lazy
fruits = ["apple" "banana" "cherry"]

"Printing fruits:"
>> fruit fruits
  fruit
<=

"Counting:"
>> item [1 2 3 4 5]
  "Number: " + $(item)
<=
```

### Example 8: Text Processing

```lazy
text = "Hello World from Lazy"
words = |(text " ")

"Words:"
words

"Word count:"
#(words)

"Join with dashes:"
&(words "-")
```

### Example 9: Temperature Converter

```lazy
c2f(celsius) =>
  f = celsius * 9 / 5 + 32
  -> f
<=

"Enter Celsius:"
? temp
fahrenheit = c2f(temp)
$(temp) + "C = " + $(fahrenheit) + "F"
```

### Example 10: Shopping List Manager

```lazy
items = []

"Welcome to Shopping List"

add_item() =>
  "Add item:"
  ? item
  items = ^(items item)
  "Added!"
<=

show_list() =>
  "Your list:"
  >> item items
    item
  <=
<=

add_item()
add_item()
add_item()
show_list()

"Total items: " + $(#(items))
```

### Example 11: Dice Roller

```lazy
roll_dice(sides) =>
  result = ?=(sides) + 1
  -> result
<=

"Rolling 6-sided dice:"
d6 = roll_dice(6)
"You rolled: " + $(d6)

"Rolling 20-sided dice:"
d20 = roll_dice(20)
"You rolled: " + $(d20)
```

### Example 12: FizzBuzz

```lazy
fizzbuzz(max) =>
  counter = 1
  @ counter <= max
    output = ""
    
    ? counter % 3 == 0
      output = "Fizz"
    <=
    
    ? counter % 5 == 0
      output = output + "Buzz"
    <=
    
    ? #(output) == 0
      $(counter)
    <=
    
    ? #(output) > 0
      output
    <=
    
    counter = counter + 1
  <=
<=

fizzbuzz(20)
```

## File Structure for Projects

**program.lazy:**
```lazy
"My Lazy Program"

main() =>
  "Starting..."
  run_program()
  "Done!"
<=

run_program() =>
  "Enter your name:"
  ? name
  "Hello " + name
<=

main()
```

## Running Your Programs

### Command Line

**Windows:**
```cmd
lazy.exe myprogram.lazy
```

**Linux/Mac:**
```bash
./lazy myprogram.lazy
```

### Create Batch File (Windows)

**run.bat:**
```batch
@echo off
lazy.exe %1
pause
```

Then:
```cmd
run.bat myprogram.lazy
```

### Create Shell Script (Linux/Mac)

**run.sh:**
```bash
#!/bin/bash
./lazy "$1"
```

Make executable:
```bash
chmod +x run.sh
./run.sh myprogram.lazy
```

## Quick Reference Card

### Symbols
```
?      Ask input OR check condition
@      Loop while condition true
>>     For-each loop through list
->     Return value from function
=>     Start function definition
<=     End block (function/loop/if)
=      Assign to variable
```

### Operators
```
#(x)       Length/count
$(x)       Convert to string  
~(x)       Convert to number
^(list x)  Push to list
v(list)    Pop from list
&(list s)  Join list with separator
|(str s)   Split string by separator
!(bool)    Boolean NOT
?=(max)     Random 0 to max-1
```

### Comparisons
```
>   Greater than
<   Less than
==  Equal to
!=  Not equal
>=  Greater or equal
<=  Less or equal (also ends blocks!)
```

### Math
```
+   Add
-   Subtract
*   Multiply
/   Divide
%   Modulo (remainder)
```

### Values
```
yes / no   Booleans
"text"     Strings  
42         Numbers
[1 2 3]    Lists
```

## Tips & Best Practices

1. **Always end blocks with `<=`** - Functions, loops, and conditionals all need it!

2. **Use descriptive variable names** - Since there are no types, names help!

3. **Space-separate list items** - `[1 2 3]` not `[1,2,3]`

4. **Context for `?`** - If it has comparison operators, it's a condition. Otherwise, it's input.

5. **Convert types explicitly** - Use `$()` for strings, `~()` for numbers

6. **Remember symbol meanings** - Visual aids help! `^` up = add, `v` down = remove

7. **One statement per line** - Keep it simple and readable

## Philosophy

**Why no keywords?**

Keywords require you to remember specific words in English. Symbols are:
- **Universal** - Work across all languages
- **Visual** - You can see what they do
- **Minimal** - Only 7 core symbols to learn
- **Intuitive** - `@` looks like a loop, `?` asks questions

**Lazy is designed for:**
- Teaching programming basics
- Quick prototyping
- Fun coding experiments
- Educational projects

## What's Not in Lazy?

Lazy deliberately excludes:
- Classes/Objects
- Imports/Modules  
- Exceptions
- Pointers
- Types

This keeps it **lazy** and **easy**!

## Error Messages

Lazy tries to be forgiving, but if something goes wrong:
- Missing `<=` - Your block won't close properly
- Wrong type - Operations return `nothing`
- Bad index - Returns `nothing`
- Missing file - Prints error message

## Limitations

- **Lists aren't nested deeply** - Keep it simple!
- **No file I/O** - Just stdin/stdout
- **No classes** - Just functions and data
- **Single file programs** - No imports

These are features, not bugs! Lazy stays lazy!

## Summary

**You just learned the world's simplest programming language!**

- 7 symbols control everything
- 10 symbol functions do common tasks
- No keywords to memorize
- Visual, intuitive syntax

Now go build something lazy! 

---

**Examples to Try:**

1. Make a calculator
2. Build a quiz game
3. Create a todo list
4. Write a story generator
5. Build a number guessing game

**Remember:** If it feels like work, you're doing it wrong! Stay lazy!
