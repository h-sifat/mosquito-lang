# Mosquito Programming Language

A toy programming language to learn how to build language interpreters.

## Grammar

### Variables

Data types:

    - Boolean
    - Number (contains both integer and floating point numbers)
    - String (only supports standard ASCII characters)
    - Character
    - Object
    - Function
    - Class
    - Enum
    - Array
    - Any
    - TypeDict

Example:

```
let isOpen: bool = false;
let value: num = 20;
let name: str = "Alex";
const asterisk: char = '*';
const MAX_LEN: num = 5;

let values: num[] = [1, 2, 3];
const mut values_2: Array<num> = [1, 2, 3]; // the array is mutable but the variable is not

let unknown: any = "hi!";

// TypeDict
let address = {
    city: "Tokio",
    zip: 23432
};

let value = {
    2 + 3
}; // 5

typeof(unknown) == "str"; // true

match typeof(unknown) {
    "str" => {
        print("it's a string!");
     },
    _ => print("something else!");
}

enum Result<V> {
    Ok(V),
    Error(str)
}

let res: Result<str> = Result::Ok("hello");

match res {
    Ok(value) => {
        print("value:", value);
    },
    Error(err) => {
        print("error:", err);
    }
}

function greet(): void { // ": void" is optional
    print("Hello World!");
}

function add(a: num, b: num): num {
    return a + b;
}

class Person {
    pub name: str;
    pub age: num;

    init(name: str, age: num) {
        self.name = name;
        self.age = age;
    }

    introduce() {
        print("Hi, I'm", self.name);
    }

    static test() {
        print("This is a static method");
    }

    get email() {
        return f"{name}@org.com"
    }
}

const mut person = new Person("Alex", 21);
person.introduce();
person.email; // "Alex@org.com"

person.age++;

Person::test();
```

## Control flow

```
if(true) {

} else {

}

if(let name = getName(); name.length < 3) {

} else if (name.length  < 5) {

} else {
    print("length:", name.length);
}

let value = age < 18 ? "minor" : "adult"; // typeof(value) == "str"

for(let i = 1, j = 10; i < j; i++, j--) {

}

while(true) {

}
```

## Arithmetic Operations

The signs remain standard as the C language

```
let value = 2 + 3;
```

The above expression internally becomes:

```
let value = Number::add(2, 3);
```
