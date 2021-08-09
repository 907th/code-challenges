import UIKit

//
// Lesson 9. Structs (part 2)
//

// Initializers
struct User {
    var name: String
    
    init() {
        name = "Anonymous"
    }
}
let anon = User()
print(anon.name)

// Self reference
struct Person {
    var name: String
    
    init(name: String = "Anonymous") {
        self.name = name
    }
}
let anon2 = Person()
print(anon2.name)

// Lazy properties
struct FamilyTree {}
struct PersonWithFamily {
    var name: String = "Anonymous"
    lazy var familyTree = FamilyTree()
}
var anon3 = PersonWithFamily()
print(anon3.familyTree)

// Static members
struct PersonWithCounter {
    var name: String
    static var count: Int = 0
    
    init(name: String = "Anonymous") {
        self.name = name
        PersonWithCounter.count += 1
    }
}
let john = PersonWithCounter(name: "John")
let petr = PersonWithCounter(name: "Petr")
print(PersonWithCounter.count)

// Visibility modifiers
struct PersonWithPrivateMember {
    private var id: Int = 0
    public var name: String = "Anonymous"
    func getId() -> String {
        return "My ID is \(id)"
    }
}
let anon4 = PersonWithPrivateMember()
print(anon4.getId())
