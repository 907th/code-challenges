import UIKit

//
// Day 13 - 14. Swift Review (parts 1 and 2)
//

// Variables and constants
var name = "John"
name = "Romeo"
let frozenName = "Lucy"
// frozenName = "123" // error!

// Data types
let str: String = "123"
let n: Int = 45
let d: Float = 1234567890.1234567890
let f: Double = 1234567890.1234567890

// Arrays
var ary = ["A", "c"]
type(of: ary)
ary.append("D")
print(ary)

// Dictionaries
var dict = ["a": "c"]
type(of: dict)
dict["Z"] = "1"
print(dict)

// Classes
class A {
    var a: Int
    init(a: Int) { self.a = a }
}
class B: A {
    var b: Int
    init(a: Int, b: Int) {
        self.b = b
        super.init(a: a)
    }
}
let b = B(a: 567, b: 123)
print(b.a, b.b)
