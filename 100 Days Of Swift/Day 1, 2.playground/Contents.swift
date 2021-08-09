import UIKit


//
// Lesson 1. Variables
//

// var - for variables
var name = "Aleksei"
var age: Int = 34

// let - for constants
let greeting = """
Hello, \(name)!
Your age is \(age)
"""


//
// Lesson 2. Arrays, dictionaries, enums
//

let beatles = ["John", "Paul", "George", "Ringo"]
beatles[1]

let colors = Set(["red", "green", "blue", "red", "blue"])

var singer = (first: "Taylor", last: "Swift", age: 30)
singer.0
singer.last

// Type of 'singer' is (first: String, last: String, age: Int). It can't be changed e.g. to:
// singer = (first: "Justin", age: 25)

// [String: Int] dictionary
let heights = [
    "Alex": 178,
    "Taylor": 163
]
heights["Taylor"]
heights["Petr"] // nil
heights["Petr", default: 100] // 100

// Empty collections
let emptyArray1 = [Int]()
let emptyArray2 = Array<Int>()
let emptyDict1 = [String: Int]()
let emptyDict2 = Dictionary<String, Int>()
let emptySet = Set<String>()

enum Result {
    case success
    case failure
}
let result = Result.success

// Enum associated values
enum Activity {
    case bored
    case running(distance: Int)
}
let act = Activity.running(distance: 127)

// Enum raw values
enum Planet: Int {
    case mercury = 1
    case venus
    case earth
    case mars
}
let planet = Planet(rawValue: 3) // earth

