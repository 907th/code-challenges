import UIKit

//
// Lesson 10. Classes
//

// Creating classes (no default initializer!)
class Dog {
    var name: String
    var breed: String
    init(name: String, breed: String) {
        self.name = name
        self.breed = breed
    }
    func makeNoise() {
        print("Woof!")
    }
}
let d1 = Dog(name: "d1", breed: "doggy")
print(d1.name, d1.breed)
d1.makeNoise()

// Inheritance, method overriding
class Poodle: Dog {
    init(name: String) {
        super.init(name: name, breed: "poodle")
    }
    override func makeNoise() {
        print("Yip!")
    }
}
let p1 = Poodle(name: "p1")
print(p1.name, p1.breed)
p1.makeNoise()

// Final classes
final class Puppy: Dog {}

// Struct objects are copied by value!
struct MyStruct { var n: Int }
var s1 = MyStruct(n: 1)
var s2 = s1
s2.n = 2
print(s1.n, s2.n)

// Class objects are copied by reference!
class MyClass {
    var n: Int
    init(n: Int) { self.n = n }
}
var c1 = MyClass(n: 1)
var c2 = c1
c2.n = 2
print(c1.n, c2.n)

// Deinitializers
class ClassDeinit {
    var n: Int
    init(n: Int) { self.n = n }
    deinit { print("Instance \(n) is being destroyed") }
}
for i in 1...3 {
    let p = ClassDeinit(n: i)
    print("Instance \(p.n) has been just created")
}

// Immutable class instance ALLOWS to change variable props (unlike structs)!
class ClassVarProp {
    var name: String
    init(name: String) { self.name = name }
}
let cvp = ClassVarProp(name: "John")
cvp.name = "Milana"
print(cvp.name)
