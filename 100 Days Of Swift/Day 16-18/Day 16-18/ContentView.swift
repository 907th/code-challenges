//
//  ContentView.swift
//  Day 16-18
//
//  Created by Alexey Chernenkov on 10.08.2021.
//

import SwiftUI

struct ContentView: View {
    let tipPercentages = [10, 15, 20, 25, 0]
    
    @State private var checkAmount = ""
    @State private var numberOfPeople = 2
    @State private var tipPercentage = 2
    
    
    var totalPerPerson: Text {
        let realNumberOfPeople = Double(numberOfPeople + 2)
        let realTipPercentage = Double(tipPercentages[tipPercentage])
        if let realCheckAmount = Double(checkAmount.replacingOccurrences(of: ",", with: ".")) {
            let realPerPerson = (realCheckAmount * (1.0 + realTipPercentage / 100.0)) / realNumberOfPeople;
            return Text("$\(realPerPerson, specifier: "%.2f") per person")
        } else {
            return Text("Enter amount!")
        }
    }
    
    var body: some View {
        NavigationView {
            Form {
                Section {
                    TextField("Amount", text: $checkAmount)
                        .keyboardType(.decimalPad)
                    Picker("Number of people", selection: $numberOfPeople) {
                        ForEach(2..<100) {
                            Text("\($0) people")
                        }
                    }
                }
                Section(header: Text("How much tip do you want to leave?")) {
                    Picker("Tip percentage", selection: $tipPercentage) {
                        ForEach(0..<tipPercentages.count) {
                            Text("\(tipPercentages[$0])%")
                        }
                    }
                    .pickerStyle(SegmentedPickerStyle())
                }
                Section {
                    totalPerPerson
                }
            }
            .navigationBarTitle("WeSplit")
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            ContentView()
        }
    }
}
