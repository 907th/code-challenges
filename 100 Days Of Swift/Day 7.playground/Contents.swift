import UIKit

//
// Lesson 7. Closures (part 2)
//

func travel(action: (String) -> Void) {
    print("I'm ready to go!")
    action("London")
    print("I'm done.")
}

travel { (city: String) in
    print("I'm going to \(city).")
}

// Returning values from closures:
func travel(action: (String) -> String) {
    print("I'm ready to go!")
    let actStr = action("London")
    print(actStr)
    print("I'm done.")
}

travel { (city: String) in
    return "I'm going to \(city)."
}

// Shorthand syntax:
travel { "I'm flying to \($0)" }

// Multiparameter closure:
func travel(action: (String, Int) -> String) {
    print("I'm ready to go!")
    let actStr = action("London", 80)
    print(actStr)
    print("I'm done.")
}

travel { "I'm going to \($0) at the speed of \($1) mph." }

// Returning closures:
func travel(city: String) -> (String) -> String {
    return { "I'm \($0) to \(city)" }
}

let t: (String) -> String = travel(city: "London")
print(t("driving"))
print(t("flying"))

// Variable capturing:
func travel(city: String) -> () -> String {
    var counter = 0
    return {
        counter += 1
        return "I'm going to \(city) for the \(counter) time"
    }
}

let t2: () -> String = travel(city: "Moscow")
print(t2())
print(t2())
print(t2())
