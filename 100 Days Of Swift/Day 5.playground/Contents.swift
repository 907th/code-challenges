import UIKit

//
// Lesson 5. Functions
//

func printHello() {
    let msg = """
Hello, stranger!
How could I help you?
"""
    print(msg)
}
printHello()

// Returning values:
func square(number: Int) -> Int {
    return number * number
}
print(square(number: 3))

// Internal/external parameter names:
func sayHello(to name: String) {
    print("Hello, \(name)!")
}
sayHello(to: "Taylor")

// Unnamed parameter:
func greet(_ name: String) {
    print("Hello, \(name)!")
}
greet("Taylor")

// Default parameter value:
func greet(_ name: String, nicely: Bool = true) {
    if nicely {
        print("Hello, \(name)!")
    } else {
        print("Oh no, it's \(name) again...")
    }
}
greet("Taylor", nicely: false)

// Variadic parameters:
func square(numbers: Int...) {
    for number in numbers {
        print("\(number)^2 = \(number * number)")
    }
}
square(numbers: 1, 2, 3)

// Throwing functions:
enum PasswordError: Error {
    case obvious
}
func checkPassword(password: String) throws -> Bool {
    if password == "password" {
        throw PasswordError.obvious
    } else {
        return true
    }
}
do {
    try checkPassword(password: "12345")
    try checkPassword(password: "password")
    print("All passwords are OK!")
} catch {
    print("Some password is bad :(")
}

// 'inout' parameters:
func changeVariable(_ variable: inout Int, to: Int) {
    variable = to
}
var n = 45
changeVariable(&n, to: 32)
print(n)

