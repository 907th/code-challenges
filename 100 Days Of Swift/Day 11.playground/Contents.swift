import UIKit

//
// Lesson 11. Protocols and extensions
//

protocol MyIdentifiable {
    var id: String { get set }
    func identify()
}

struct User: MyIdentifiable {
    var id: String
    func identify() {
        print("ID: \(id)")
    }
}

func displayId(smth: MyIdentifiable) {
    smth.identify()
}

let user = User(id: "John")
displayId(smth: user)

// Multiple inheritance
protocol A { func a() }
protocol B { func b() }
protocol AB: A, B {}

// Class extensions
extension Int {
    func squared() -> Int {
        return self * self
    }
    
    // Computed props ONLY!
    var isEven: Bool {
        return self % 2 == 0
    }
}
let n = 5.squared()
print(n)
print(n.isEven)

// Protocol extensions
extension Collection {
    func summarize() {
        print("My size is \(count)")
        for v in self {
            print(v)
        }
    }
}
let x: Array<Int> = [1, 4, 6]
let s: Set<String> = Set(["a", "b", "c"])
x.summarize()
s.summarize()

// Protocol Oriented Programming (POP) example
extension MyIdentifiable {
    func identify() {
        print("My ID is \(id)")
    }
}

struct User2: MyIdentifiable {
    var id: String
}

let u2 = User2(id: "user2")
u2.identify()

