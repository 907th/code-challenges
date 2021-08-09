import UIKit

//
// Lesson 6. Closures (part1)
//

let clos = {
    print("I'm a closure!")
}

clos()

// With parameter:
let clos2 = { (str: String) in
    print("str = \(str)")
}

clos2("123")

// With return value:
let clos3 = { (str: String) -> String in
    return "Hello, \(str)!"
}

print(clos3("John"))

// As parameter:

func quest(greet: () -> Void) {
    greet()
    print("How are you?")
}

quest(greet: clos)

// Trailing closure syntax:

quest {
    print("Hi, Lisa!")
}
