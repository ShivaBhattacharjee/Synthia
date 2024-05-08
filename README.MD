<p align="center">
  <a href="https://github.com/ShivaBhattacharjee/Synthia">
    <img src="./assets/synthia.png" alt="Logo" width="150px" >
  </a>
<br/>
  <h3 align="center">Synthia</h3>
  <p align="center">
    A high level interpreted language built on top of rust
    <br/>
    <br/>
    <a href="https://hub.docker.com/r/immashiva/synthia">Docker</a>
    .
    <a href="https://github.com/ShivaBhattacharjee/synthia/issues">Report Bug</a>
    .
    <a href="https://github.com/ShivaBhattacharjee/synthia/issues">Request Feature</a>
  </p>
</p>

## Table Of Contents

- [Table Of Contents](#table-of-contents)
- [About The Project](#about-the-project)
  - [About Synthia](#about-synthia)
    - [Key Features:](#key-features)
- [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
- [Installation](#installation)
  - [Docker](#docker)
  - [Local](#local)
- [Syntax](#syntax)
  - [How to execute Synthia](#how-to-execute-synthia)
    - [Executing through IDE and file system](#executing-through-ide-and-file-system)
    - [Alternatively You can execute code in repl by](#alternatively-you-can-execute-code-in-repl-by)
  - [Hello world](#hello-world)
    - [1. `out` Function](#1-out-function)
    - [2. `bigPrint` Function](#2-bigprint-function)
  - [Variables](#variables)
    - [Example](#example)
    - [Strings](#strings)
    - [Integers](#integers)
    - [Floats](#floats)
  - [Functions](#functions)
    - [Example](#example-1)
    - [How to use](#how-to-use)
    - [⚠️  Important Note](#️--important-note)
  - [Condition](#condition)
    - [Example](#example-2)
    - [How the code works](#how-the-code-works)
      - [Variable Initialization:](#variable-initialization)
      - [Conditional Statement:](#conditional-statement)
      - [Output:](#output)
      - [Else Block:](#else-block)
    - [Important Notes](#important-notes)
  - [Objects](#objects)
    - [Object Definition](#object-definition)
    - [Accessing Object Properties](#accessing-object-properties)
    - [Output](#output-1)
    - [Important Notes](#important-notes-1)
  - [Arrays](#arrays)
    - [Array Definition](#array-definition)
    - [Accessing Array Elements](#accessing-array-elements)
    - [Output](#output-2)
    - [Important Notes](#important-notes-2)
- [Contributing](#contributing)
  - [Creating A Pull Request](#creating-a-pull-request)
- [Raising an issue](#raising-an-issue)
- [License](#license)
- [Authors](#authors)

## About The Project

<img src="./assets/Synthia-Banner.png">

### About Synthia

Synthia is a programming language crafted on the robust foundation of Rust, tailored for individuals entering the world of programming, especially those in academic settings. It was designed with the intention of making programming accessible to university students who are keen to start their coding journey but may feel daunted by complex syntax and intricacies.

#### Key Features:

1. **Built on Rust Backbone:**
   - Synthia builds upon the reliable and efficient Rust programming language. By inheriting Rust's core principles of performance, memory safety, and concurrency, Synthia provides a solid and secure environment for programming endeavors.

2. **Faster Than Python:**
   - Synthia delivers superior performance compared to interpreted languages like Python. This speed advantage is crucial for handling computational tasks and projects with resource-intensive requirements.

3. **Easier Syntax than C++:**
   - Synthia takes inspiration from Rust's syntax, ensuring a balance between simplicity and expressiveness. It is designed to be more approachable for beginners, offering an easier learning curve compared to the intricate syntax of languages like C++.

4. **Tailored for University Folks:**
   - Synthia caters specifically to university students who are dipping their toes into the world of programming. The language aims to alleviate the initial apprehension often associated with syntax complexities, allowing students to focus on understanding fundamental programming concepts.

5. **User-Friendly Approach:**
   - With a user-friendly syntax, Synthia strives to create a welcoming environment for newcomers. This approach enables aspiring programmers to grasp programming concepts without being overwhelmed by unnecessary intricacies.

Synthia is not just a programming language; it is a supportive tool for individuals taking their first steps into the exciting realm of coding. By blending the power of Rust with an emphasis on simplicity, Synthia empowers university students to unlock their programming potential with confidence.

## Built With

Synthia is built on top of Rust

- [Rust](https://www.rust-lang.org/)

## Getting Started

### Prerequisites

<a href="https://git-scm.com/downloads" >Git</a> is a distributed version control system used for software development. It allows multiple developers to work on the same codebase simultaneously, keeping track of changes and managing versions. It also enables users to revert changes and collaborate more effectively.

<a href="https://nodejs.org/en/download/">Rust</a> is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety ensuring that all references point to valid memory without requiring the use of a garbage collector or reference counting present in other memory-safe languages. To simultaneously enforce memory safety and prevent data races, its "borrow checker" tracks the object lifetime of all references in a program during compilation. Rust borrows ideas from functional programming, including immutability, higher-order functions, and algebraic data types. It is popular for systems programming

## Installation

  ### Docker
```sh
docker run immashiva/synthia
```
  ### Local
```sh
git clone https://github.com/ShivaBhattacharjee/Synthia
```

```sh
cd Synthia
```

To run the compiler in terminal

```
cargo run
```

## Syntax

### How to execute Synthia
 #### Executing through IDE and file system
1. Make a file with .syn extension
```bash
$ touch helloworld.syn
```
2. Write the following code 
```bash
out("Hello world from Synthia!!");
``` 
3. Type the following command from terminal

```bash
$ cargo run run helloworld.syn 
```
4. Output
```bash
 Hello world from Synthia!!
```
  #### Alternatively You can execute code in repl by
  ```bas
  cargo run
  ```

  ### Hello world

  In Synthia, there are two primary functions for printing output: `out` and `bigPrint`. Each serves a specific purpose in displaying information.

  #### 1. `out` Function

  The `out` function is a versatile printing tool in Synthia. It can be used to print individual values, concatenate strings, and display the contents of objects. Here's an example:

  ```sh
  make greeting = "Hello";
  out(greeting, "World!");
  ```
  
  #### 2. `bigPrint` Function
  The bigPrint function is designed to handle heavy data, such as printing all elements of an array or large datasets. It is particularly useful when dealing with extensive information that might be impractical for standard output. Here's an example:

  ```sh
  make dataArray = [1, 2, 3, 4, 5];
  sbigPrint(dataArray);
  ```

  ### Variables

  To declare a variable in Synthia, use the `make` keyword.

  #### Example

  ```sh
  make name = "example"
  ```
  Synthia is a dynamically typed language, so there is no need to explicitly mention the data type.

  #### Strings

  Strings can be assigned using double quotes (""), while for integers or floats, you can simply mention the value without quotes.

  #### Integers
  ```sh
  make number = 12
  ```
  #### Floats

  ```sh
  make anothernum = 12.5
  ```
  In Synthia, the language dynamically interprets the type based on the assigned value. Therefore, you don't need to specify the type explicitly.
  
  ### Functions

  In Synthia, functions are declared using the `func` keyword. Here's an example of a function that calculates the sum of two values and prints the result:
    
  #### Example
  ```sh
  make sum = func(a, b) {
    make result = a + b;
    out("The sum is", result);
 };
  ```
  #### How to use
  ```sh
  sum("1", "2");
  sum(9, 10);
  ```
  In the first example, the function is called with string arguments, and in the second example, it's called with integer arguments. Synthia's dynamically typed nature allows you to use the function with different data types without explicitly specifying them.

  #### ⚠️  Important Note
  In Synthia, function arguments are dynamically typed, but it's essential to ensure that the function is called with compatible data types for the expected behavior. In the example above, the sum function assumes that the arguments are numeric and can be added. Always be mindful of the expected input types when designing and using functions

  ### Condition
  In Synthia, conditional statements allow you to execute different blocks of code based on whether a given condition is true or false. Here's an example illustrating the use of a basic `if-else` statement:

  #### Example

  ```sh
  make x = 10;
  make y = 5;
  make z = 9.1;

  if (x > y) {
    out("x is greater than y!");
    out(z);
  } else {
    out("x is not greater than y!");
  }
  ```

  #### How the code works
  
  ##### Variable Initialization:
  x, y, and z are initialized with the values 10, 5, and 9.1 respectively.

  ##### Conditional Statement:

  The if (x > y) statement checks whether the value of x is greater than the value of y.

  ##### Output:

  If the condition is true, it executes the block within the curly braces ({}) under the if statement.
  Outputs the message "x is greater than y!" using the out function.
  Outputs the value of z using the out function.
  
  ##### Else Block:

  If the condition is false, it executes the block within the curly braces under the else statement.
  Outputs the message "x is not greater than y!" using the out function.
  
  #### Important Notes
   i. Ensure that the condition inside the if statement evaluates to a boolean result (true or false).

  ii. The out function is used for printing messages and values to the console

  ### Objects
  
  In Synthia, objects are key-value pairs enclosed in curly braces `{}`. Each key is associated with a value, forming a collection of properties. Here's an example of defining and accessing an object:

  ```sh
  make testObj = {
  "firstKey": "Hello",
  "secondKey": "World",
  };
  out(testObj's firstKey, testObj's secondKey);
```
   #### Object Definition
  
  Objects in Synthia use the {} syntax, where each property consists of a key and a corresponding value. Keys and values are separated by a colon :. In the example above, testObj is an object with two properties: firstKey and secondKey, each associated with a string value.

  #### Accessing Object Properties
  To access the properties of an object, use the syntax object's key. In the example, testObj's firstKey and testObj's secondKey are used to retrieve the values associated with the respective keys.

  #### Output
  The out function is used to print the values of the object's properties. In the provided example, it prints the values associated with firstKey and secondKey, resulting in the output.

  ```sh
  Hello World
  ```
 #### Important Notes
  i. Keys in Synthia objects are enclosed in double quotes (").

  ii. Ensure that the keys used to access object properties match the actual keys in the object.

  ### Arrays
  In Synthia, arrays are used to store collections of elements. They are created using square brackets `[]` and can hold values of any type. Here's an example of defining and using an array:

  ```sh
  make Synthia = ["Faster", "Simpler", "OpenSource"];
  out(Synthia);
  ```
  #### Array Definition
  Arrays in Synthia use square brackets [] to enclose a list of elements. Elements within the array are separated by commas. In the provided example, the Synthia array contains three string elements: "Faster," "Simpler," and "OpenSource."

  #### Accessing Array Elements
  To access individual elements in an array, you can use their position, or index, within the array. In Synthia, array indices start from 0. For example:

  ```sh
  out(Synthia[0]);  # Outputs "Faster"
  out(Synthia[2]);  # Outputs "OpenSource"
  ```
  #### Output
  The out function is used to print the entire array or specific elements. In the provided example, it prints the entire Synthia array, resulting in the output:

  #### Important Notes
  i. Array indices start from 0, so the first element in the array has index 0, the second has index 1, and so on.
  
  ii. Ensure that the elements in the array are of compatible types.


## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

- If you have suggestions for adding or removing projects, feel free to [open an issue](https://github.com/ShivaBhattacharjee/synthia/issues) to discuss it

- Please make sure you check your spelling and grammar.

### Creating A Pull Request

Wanna contribute to Synthia ?

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/FeatureName`)
3. Commit your Changes (`git commit -m 'Add some FeatureName'`)
4. Push to the Branch (`git push origin feature/FeatureName`)
5. Open a Pull Request

## Raising an issue

If you're experiencing any problems with Synthia, please be sure to review our [issue template](https://github.com/ShivaBhattacharjee/synthia/tree/main/.github/ISSUE_TEMPLATE) before opening a new issue. The template includes a list of questions and prompts that will help us better understand the issue you're experiencing, and it will ensure that we have all of the necessary information to investigate the problem.

We kindly ask that you provide as much detail as possible when submitting an issue, including steps to reproduce the problem, any error messages that you have seen, and any other relevant information. This will help us to identify and fix the issue more quickly.

Thank you for your cooperation, and we look forward to hearing from you!

## License

Distributed under Apache License . See [LICENSE](https://github.com/ShivaBhattacharjee/synthia/blob/main/LICENSE) for more information

## Authors

- **Shiva Bhattacharjee** - [Shiva Bhattacharjee](https://github.com/ShivaBhattacharjee)
