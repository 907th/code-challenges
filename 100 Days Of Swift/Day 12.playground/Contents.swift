import UIKit

//
// Lesson 12. Optionals
//

var age: Int? = nil

// Unwrap with 'if let'
if let uAge = age {
    print("Age is set! It equals to \(uAge)")
} else {
    print("Age is not known!")
}

// Unwrap with 'guard let'
func printBirthYear(age: Int?) {
    guard let uAge = age else {
        print("Birth year is not known!")
        return
    }
    print("Birth year is \(2021 - uAge)")
}
printBirthYear(age: age)

// Force unwrapping
let str = "5"
let strIOpt = Int(str)
print(strIOpt)
let strI = Int(str)!
print(strI)

// Implicitly unwrapped optionals:
// they are optionals, but DON'T need to be unwrapped explicitly when used
let numImpOpt: Int! = 123
print(numImpOpt)
print("Squared = \(numImpOpt * numImpOpt)")

// nil coalescing operator '??'
print(strIOpt ?? "Some default value")
print(age ?? 123)

// Optional chaining with '?'
let ary = ["abc", "def", "gh"]
print(ary.first?.uppercased())
print(age?.negate())

//
// Wrapping throwing func into optionals
//
enum MyErr: Error {
    case err
}
func thf(_ id: Int) throws -> String {
    if (id < 0) { throw MyErr.err }
    return String(id)
}
print(try? thf(-1)) // nil
print(try? thf(1)) // Optional(1)
// print(try! thf(-1)) // run time error!
print(try! thf(1)) // 1

// Failable initializer
class Person {
    var id: String
    init?(num: Int) {
        if num < 0 {
            return nil
        } else {
            self.id = String(num)
        }
    }
}
let john = Person(num: -4) // Person?
print(john) // nil

// Typecast with 'as?'
class Z {}
class Z1: Z { func z1() { print("Z1") } }
class Z2: Z { func z2() { print("Z2") } }
let zAry = [Z1(), Z1(), Z2(), Z2(), Z1()]
for z in zAry {
    if let z1 = z as? Z1 { z1.z1() }
    if let z2 = z as? Z2 { z2.z2() }
}
