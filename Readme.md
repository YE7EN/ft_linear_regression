The project is a linear regression, we teach a program to predict a car's price based on its mileage.
The math is simple : find the best line through a scatte of points.

that line has the equation:
    price = θ0 + θ1 x mileage

θ0(theta0)= where the line crosses the y-axis (price at 0km)
θ1(theta1)= the slope (how much price changes per km)

you have two programs:
    train -> reads the data, figures out the best θ0 and θ1, saves them
    predict -> reads θ0 and θ1, asks you for a mileage, gives you a price

plus two bonus progams:
    precision -> measures how accurate the model is (R²)
    plot -> draws the scatter plot + regression line as a PNG

cargo.toml

We can think of this like a Makefile + package manager config combined

[package] is metadata. The name field is important it's what you use when importing your own library int the binaries: use ft_linear_regression::...

edition = "2024" is the Rust language version. Like C89/C99/C11, it controls wich language features are available. We use 2021 for compatibility with older Rust versions on school machines.

[dependencies] is where you declare external libraries. Cargo downloads them automatically from crates.io ( Rust's equivalent of npm). The version string "1.3" means "version 1.3 or any compatible newer version".

Why no serde ? We could have used it to auto-deserialize CSV rows into structs, but parsing manually with .parse() is simpler and educational enough for this project.


src/lib.rs - the shared module

This is the heart of the project. All the business logic lives here. The binaries (train, predict, etc.) just call these functions.

Imports

use std::error::Error;
use std::fs;

use is like #include in C but more precise - you import exactly what you need. std is Rusts' standard library, always available without declaring it in Cargo.toml.

std::error::Error is a trait (we'll get to traits in a second) that represents "any kind of error". std::fs contains file system functions like read_to_string and write.

The DataPoint struct

pub struct DataPoint {
    pub mileage: f64,
    pub price: f64,
}

exactly like C struct - a named grouping of data. One DataPoint = one row of the CSV.

pub means public - visible outside this module. In Rust everything is private by default.
You need pub on both the struct itself AND each field, otherwise the binaries can't acces them.

f64 is a 64-bit floating point number - the equivalent of double in C.
We use f64 over f32 (32-bit) for precision - the gradient descent calculations accumulate small errors and 64 bits keeps those errors negligible.












load_dataset - reading the CSV

pub fn load_dataset(path: &str) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mileage: f64 = record[0].parse()?;
        let price: f64 = record[1].parse()?;
        data.push(DataPoint { mileage, price});
    }
    Ok(data)
}

the signature - breaking it down piece by piece

&str

In Rust there are two string types:

String - owned, heap-allocated, modifiable. Like a malloc'd char array in C.
&str - a borrowed reference to a string, read-only.

we use &str for the path argument because we just want to read it - we don't need to own it or modify it, When you call load_dataset("data.csv"), the string literal "data.csv" is a &str.

Rule of thumb: function argument that are strings -> use &str. Stroring a string in a struct -> use String.

Result<Vec<Data_point>, Box<dyn Error>>

This is the return type. Rust has no exceptions. Instead, any function that can fail returns a Result. It's an enum with two variants:

Ok(value)  -> succes - here value is a Vec<DataPoint>
Err(error) -> failure - here error is a Box<dyn Error>

Vec<DataPoint> is a dynamic array of DataPoints - like std::vector in C++ or a Python list. it grows automatically as you push elements.

Box<dyn Error> means "any type of error, heap-allocated". Breaking it down:
    Box is a heap pointer - like malloc but automatic and safe
    dyn error is a trait object - "some type that implements the Error trait, I don't care which one"

    We use this because multiple different error types can occur in this function: a file-not-found error, a CSV parsing error, a float conversion error - they're all different types but all implement Error. Box<dyn Error> lets us return any of them without specifying which.

    The ? operator


    this is the most important error-handling tool in Rust. Every time you see ? at the end of a line:
        If the result is Ok(value) -> unwrap the value and continue
        If the result id Err(e) -> immediately return the error to the caller

    so this:
    let mut reader = csv::Reader::from_path(path)?;
    is equivalent to this in C-style:
    reader = csv_open(path);
    if (reader == ERROR) return ERROR;

    without ? you'd have to write:
    let mut reader = match csv::reader::from_path(path) {
        Ok(r) => r,
        Err(e) => return Err(Box::new(e)),
    };

    ? collapses that into one character. You'll see it constantly in rust code.

    The body

    let mut data = Vec::new();

    Creates an empty Vec. let mut because we're going to push elements into it - everything in Rust is immutable by default. Without mut, the compiler refuses to let you midify it.
    This is ont of Rust's core safety guarantees: if you don't declare something mutable, nobody can accidentally modify it.

    for result in reader.record() {
        let record = result?;
    }

    reader.record() returns an iterator over the CSV rows. The header row (km, price) is automatically skipped by the csv crate. Each item in the iterator is itslef a Result - because reading each line can fall independently. So result? unwraps it or propagates the error.

    let mileage: f64 = record[0].parse()?;
    let price: f64 = record[1].parse()?;

    record[0] is teh first column (a string like "240000").parse() converts it to a number.
    The : f64 type annotation tells the compile what type to parse into - withoutit, the compile doesn't know if you want f32, f64, i32, etc. The ? handles the case where the string isn't a valid number.

    data.push(DataPoint { mileage, price });

    DataPoint { mileage, price} is shorthand for DataPoint { mileage: mileage, price: price }.
    When the variable name matches the filed name, Rust lets you write it once.
    
    Ok(data)

    The last line with no semicolon - this is the return value. In Rust, the last expression in a function (without ;) is automatically returned. We wrap it in Ok() because the return type is Result.

    The rule:semicolon = statement (no value). No semicolon = expression(has a value, gets returned).





estimate_price


pub fn estimate_price(mileage: f64, theta0: f64, theta1: f64) -> f64 {
    theta0 + theta1 * mileage
}

This one is straightforward - it's directly the formula from the subject:

estimatePrice(mileage) = θ0 + θ1 x mileage

The only Rust thing to notice: no semicolon on the last line - so theta0 + theta1 * mileage is the return value. No return keyword needed.

This function is used in two places:
    In predict -> to give the user a price
    In train -> dureing every iteration of the gradient descent to measure how wrong the current thetas are

load_thetas

pub fn load_thetas(path: &str) ->(f64, f64) {
    match fd::read_to_string(path) {
        Ok(content) => {
            let parts: Vec<&str> = content.trim().split(',').collect();
            if parts.len() != 2 {
                return (0.0, 0.0);
            }
            let theta0 = parts[0].parse().unwrap_or(0.0);
            let theta1 = parts[1].parse().unwrap_or(0.0);
            (theta0, theta1)
        }
        Err(_) => (0.0, 0.0),
    }
}

Tuples - (f64, f64)

A tuple is a fixed-sized group of values. (f64, f64) is a pair of two floats. You access elements with .0 and .1:

let t = (8477.0, -0.021);
println!("{}", t.0); // 8477.0
println!("{}", t.1); // -0.021

We use a tuple here instead of creating a dedicated struct because it's just two floats - not worth the overhead of naming a struct. The caller destructures it immediately anyway:

let (theta0, theta1) = load_thetas("thetas.txt");

This is destructuring - extracting both values from the tuple in one line. Very common in Rust.

match - pattern matching

match is one of rust's most powrful features> think of it as a switch in C far more expressive.
The key rule: every possible case must be handled - the compile enforces this.

Here we match on the Result returned by fs::read_to_string(path);

match fs::read_to_string(path) {
    Ok(content) => { ... } // file exists -> content has the file text
    Err(_) => (0.0, 0.0), // file doesn't exist -> return default thetas
}

The Err(_) arm: the _ means "I don't care about the actual error value". Whether the file doesn't exist, or we don't have permission to read it - in both cases we just return (0.0, 0.0). this is jow we implement the subject's requirement: "before running training, theta0 and theta1 will be set to 0"

Inside the Ok arm

let parts: Vec<&str> = content.trim().split(',').collect();

This is a method to chain - each method trasfomrs the data and passes it to the next:

content.trim() -> removes whitespaces and \n from start and end of the string
.split(',') -> splits on commas -> produces an iterator of &str slices
.collect() -> consumes the iterator and collects results into a collection

The : Vec<&str> type annotation on parts is required here. .colelct() is generic - it can produce a Vec, a HashSet, a String, and more.
Without the annotation, the compiler can't know wich one you want and will refuse to compile.

&str inside the Vec - Why not String ? Because split doesn't create new strings, it creates views(slices) into the orginal content string. No allocation needed. &str is just a pointer + lenght pointing into content's memory.

if parts.len() != 2 {
    return (0.0, 0.0);
}

Defensive check - if the file is malformed (empty, or has the wrong format), we fall back to defaults. The return here is an early return - one of the rare cases where you use return explicitly in Rust (because it's not the last expression).

let theta0 = parts[0].parse().unwrap_or(0.0);
let theta1 = parts[1].parse().unwrap_or(0.0);

.parse() converts the string to f64. No type annotations needed here - Rust infers f64 from the contect (we're storing into variables that will be returned as (f64, f64)).

.unwrap_or(0.0) - "give me the value if Ok, otherwise give me 0.0"Softer than .unwrap() wich could crash the program on error.

(theta0, theta1)

The full flow of load_thetas
"thetas.txt" exists?
      ↓ yes                    ↓ no
read content                    return (0.0, 0.0)
trim + split + collect
exactly 2 parts?
  ↓ yes                         ↓ no
parse both                      return (0.0, 0.0)
return (theta0, theta1)

Two concepts to make sure we are clear before we move to the most important function (train):

1-Destructuring:

let (theta0, theta1) = load_thetas("thetas.txt");
// theta0 and theta1 are now two separates f64 variables

2-match vs if/else:
match is exhaustive - if you forget a case, the compile refuses to compile. This is a huge safety advantage over C's switch whre forgetting a case is silently nothing.


train - The gradient descent

pub fn train(data: &[DataPoint], learning_rate: f64, iterations: usize) -> (f64, f64) {
    let m = data.len() as f64;

    let km_min = data.iter().map(|p| p.mileage).fold(f64::INFINITY, f64::min);
    let km_max = data.iter().map(|p| p.mileage).fold(f64::NEG_INFINITY, f64::max);

    let normalized: Vec<(f64, f64)> = data
        .iter()
        .map(|p| ((p.mileage - km_min) / (km_max - km_min), p.price))
        .collect();

    let mut theta0 = 0.0_f64;
    let mut theta1 = 0.0_f64;

    for _ in 0..iterations {
        let mut sum0 = 0.0;
        let mut sum1 = 0.0;

        for &(km, price) in &normalized {
            let error = estimate_price(km, theta0, theta1) - price;
            sum0 += error;
            sum1 += error * km;
        }

        let tmp0 = learning_rate * (sum0 / m);
        let tmp1 = learning_rate * (sum1 / m);

        theta0 -= tmp0;
        theta1 -= tmp1;
    }

    let denorm_theta1 = theta1 / (km_max - km_min);
    let denorm_theta0 = theta0 - theta1 * km_min / (km_max - km_min);

    (denorm_theta0, denorm_theta1)
}


Before the code, let's understand what his function is trying to do.

The goal
You have 24 points (km, price). You want to find the line price = θ0 + θ1 x km that fits them best.

But how do you find the best θ0 and θ1? You can't just guess. The gradiant descent algorithm does this:

1. Start with θ0 = 0, θ1 = 0 (a flat line at zero)
2. Mesure how wrong that line is.
3. Adjust θ0 and θ1 slightly in the direction that reduces the error4.
4. Repeat 1000 times.

After 1000 iterations, the line has "slid" into the best position through the data.

Visually:
iteration 0: flat line at zero, terrible
iteration 10: starting to tilt
iteration 100: getting close
iteration 1000: best fit line

The parameters

data: &[DataPoint]

&[DataPoint] is a slice - a borrowed view over a sequence of DataPoint. The & means we borrow the data without taking ownership. The function can read the points but can't modify or free them.

Why slice instead of Vec ? A slice is more flexible - it works with any contiguous sequence of DataPoints, whether it comes from a Vec, a fixed array, or a portion of another array. It's the idiomatic Rust way to say "give me read access to a sequence".

iterations: usize

usize is the unsigned integer type that matches the machine's pointer size - 64 bits on a 64-bit machine. In Rust, anything that represents a count or index uses usize. Loop counters, Vec::len(), array indices - all usize.

Step1- m

let m = data.len() as f64;

data.len() returns the number of points - usize. We need it as f64 for division later.
as f64 is an explicit cast - Rust never converts type implicitly. You always have to say it explicitly, unlike C where int / double silently promotes.

m = 24.0 in our case.

step2 - finding min and max

let km min = data.iter().map(|p| p.mileage).fold(f64::INFINITY, f64::min);
let km_max = data.iter().map(|p| p.mileage).fold(f64::NEG_INFINITY, f64::max);

Let's break this chain down step by step:

data.iter() - creates an iterator over the DataPoints. Doesn't do anything yet, jsut sets up the traversal.

.map(|p| p.mileage) - transforms each DataPoint int ojust its mileage. The |p| is a closure - an anonymous function. Like a python lambda:

Now we have an iterator of f64 mileage values.

.fold(f64::INFINITY, f64::min) - this is the interesting one. fold reduces an entire iterator to a single value. It takes:
- A starting value -> f64::infinity (positive infinity)
- A function to apply at each step -> f64::min (returns the smaller of two values)

it works like this:

start:          current = INFINITY
see 240000:     current = min(INFINITY, 240000) = 240000
see 139800:     current = min(240000, 139800)   = 139800
see 150500:     current = min(139800, 150500)   = 139800
... and so on for all 24 points
result: the smallest mileage in the dataset

We start at +∞ so taht the first real value always win. For km_max we start at NEG_INFINITY so the first real value wins in the other direction.


step 3 - normalization

let normalized: Vec<(f64, f64)> = data
    .iter()
    .map(|p| ((p.mileage - km_min) / (km_max - km_min), p.price))
    .collect();

For each DataPoint we apply the min-max formula:

km_normalized = (km - min) / (max - min)

with our dataset: min ≈ 20000, max ≈ 240000:

km = 240000 -> (240000 - 20000) / (240000 - 20000)  = 1.0
km = 20000  -> (20000 - 20000) / (240000 - 20000)   = 0.0
km = 130000 -> (130000 - 20000) / (240000 - 20000)  = 0.5

Every km value is now between 0.0 and 1.0. The price stays untouched.

The results is a Vec<(f64, f64)> - a vector of tuples (normalized_km, price).

Why is this necessary? Without normalization:

error x km = 500 x 240000 = 120,000,000

That massive number gets added to theta1 every iteration -> theta1 explodes to infinity -> you get NaN. Whit normalization:

error x km_norm = 500 x 0.8 = 400

Small, manageable. The gradient descent converges smoothly.

Step 4 - the gradient descent loop

let mut theta0 = 0.0_f64;
let mut theta1 = 0.0_f64;

for _ in 0..iterations {
    let mut sum0 = 0.0;
    let mut sum1 = 0.0;

    for &(km, price) in &normalized {
        let error = estimate_price(km, theta0, theta1) - price;
        sum0 += error;
        sum1 += error * km;
    }

    let tmp0 = learning_rate * (sum0 / m);
    let tmp1 = learning_rate * (sum1 / m);

    theta0 -= tmp0;
    theta1 -= tmp1;
}

0.0_f64 - the _f64 suffix explicitly marks the literal as a 64-bit float. Same as 0.0 in this context, just extra explicit.

for _ in 0..iterations - 0..iterations is a range, like range(1000) in python. The _ means "I don't nedd the counter value, just loop 1000 times".

Inside the outer loop - the inner loop:

for &(km, price) in &normalized {}

&normalized - we borrow the vec (don't consume it). &(km, price) - we destructure each tuple directly into two variables. The & in front dereferences the reference so we get copied f64 values (f64 is a primitive, copying it is free and automatic).

let error = estimate_price(km, theta0, theta1) - price;

This is the core measurement: how wrong is our current line for this point?
    - If our line predicts 6000 but the real price is 7000 -> error = -1000 (we predicted too low)
    - If our line predicts 8000 but the real price is 6000 -> error = +2000 (we predicted too high)

sun0 += error;
sum1 += error * km;

We accumulate the errors across al 24 points. sum1 weighs the error by th e km valuebecause theta1 controls the slope - points with higher km have more influence on the slope.

After the inner loop - the update:

let tmp0 = learning_rate * (sum0/m);
let tmp1 = learning_rate * (sum1 / m);

theta0 -= tmp0;
theta1 -= tmp1;

sum / m = average error across all points. * learning_rate = scale the step size (0.1 here - don't move too fast or too slow).

Why tmp0 and tmp1 first, then update?

This is the simultaneaous update the subject explicitly requires. If you did:

theta0 -= learning_rate * (sum0 / m); // theta0 is now new
theta1 -= learning_rate * (sum1 / m); // but sum1 was calculated with OLD theta0

The second update would use a mix of old theta0 (in sum1) and new theta0 (just updated).
That's mathematically wrong. By computing both tmp values first , both updates use the same old thetas.


The intuition of one full itreration:

current line is wrong
    ↓
measure how wrong it is on all 24 points
    ↓
compute the average error direction
    ↓
nudge theta0 and theta1 slightly to reduce that error
    ↓
repeat

After 1000 nudges, the line has found its best position.

Step 5 - Denormalization

let denorm_theta1 = theta1 / (km_max - km_min);
let denorm_theta0 = theta0 - theta1 * km_min / (km_max - km_min);

We trained with normalized km values (0 to 1). But predict receives real km values (20000 to 240000. We need to convert the thetas so they work on raw km.

The math: we trained the line:

price = theta0 + theta1 x km_norm

We substitute km_norm = (km - min) / (max - min) and expand:

price = theta0 + theta1 × (km - min) / (max - min)
      = [theta0 - theta1 × min/(max-min)] + [theta1/(max-min)] × km

So the new coefficient for raw km are:

new_theta1 = theta1 / (max - min)
new_theta0 = theta0 - theta1 x min / (max - min)

After this, predict can just do theta0 + theta1 x 100000 with a real km value and get the right answer - it never needs to know about normalization.

The full picture of train

raw data (km: 20000-240000)
    ↓ find min/max
    ↓ normalize km to 0-1
    ↓ gradient descent 1000 times
        each iteration:
            measure error on all 24 points
            nudge theta0 and theta1 toward less error
    ↓ denormalize thetas back to raw km scale
return (theta0, theta1)

