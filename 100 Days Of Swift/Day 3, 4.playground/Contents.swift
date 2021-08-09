import UIKit

//
// Lesson 3. Operators
//

let a = 12
let b = 32

a + b
a - b
a * b
a / b
a % b

// Operator + is overloaded for other types:
"a" + "b"
[1, 2] + [3, 4]

// Compound assignment operators:
var x = 45
x += 1
x -= 2
x /= 3
x %= 4

// Comparison operators:
a == b
a != b
"a" < "z"

// 'if' statement:
if a == b || abs(a - b) == 1 {
    print("a almost equals b")
} else {
    print("a not equals b")
}

// Ternary operator:
a == b ? "a eq b" : "a not eq b"

// `switch` statement:
switch a {
case 11: print("eleven")
case 12: print("twelve"); fallthrough
default: print("a > 11!")
}

// Ranges:
1..<5 // half-open
1...5 // closed

let c = 10 // two digit
switch c {
case 0..<10: print("one digit")
case 10..<100: print("two digit")
default: print("more than two digit")
}


//
// Lesson 4. Loops
//

let count = 1...3;
for c1 in count {
    print("\(c1)...")
}
print("Go!")

for _ in 1...3 {
    print("Hi!")
}

var n = 1
while n < 4 {
    n += 1
}
print(n)

repeat {
    print("Execute at least once!")
} while false

var e = 0;
while true {
    e += 1
    if e > 4 { break }
}
print(e)

outerLoop: for a1 in 1...9 {
    for a2 in 1...9 {
        if a1 * a2 == 42 {
            print("\(a1) * \(a2) = 42")
            break outerLoop
        } else {
            continue
        }
    }
}
