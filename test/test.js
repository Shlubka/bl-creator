// Объявление переменных
let a = 10;
const b = 20;
var c = 30;

// Арифметические операции
let sum = a + b + c;
let diff = a - b - c;
let product = a * b * c;
let quotient = a / b / c;
let remainder = a % b % c;

// Логические операции
let isTrue = (a > b) && (c < b);
let isFalse = (a < b) || (c > b);
let isNot = !(a === b);

// Условные операторы
if (a > b) {
    console.log("a is greater than b");
} else if (a < b) {
    console.log("a is less than b");
} else {
    console.log("a is equal to b");
}

// Циклы
for (let i = 0; i < 5; i++) {
    console.log("For loop iteration:", i);
}

let j = 0;
while (j < 5) {
    console.log("While loop iteration:", j);
    j++;
}

let k = 0;
do {
    console.log("Do-while loop iteration:", k);
    k++;
} while (k < 5);

// Функции
function add(x, y) {
    return x + y;
}

const subtract = (x, y) => {
    return x - y;
};

console.log("Addition:", add(a, b));
console.log("Subtraction:", subtract(a, b));

// Объекты
let person = {
    name: "John",
    age: 30,
    greet: function() {
        console.log("Hello, " + this.name);
    }
};

person.greet();

// Массивы
let numbers = [1, 2, 3, 4, 5];
numbers.forEach(number => {
    console.log("Number:", number);
});

// Исключения
try {
    throw new Error("This is a test error");
} catch (e) {
    console.error("Caught an error:", e.message);
} finally {
    console.log("This will always run");
}

// Классы
class Animal {
    constructor(name) {
        this.name = name;
    }

    speak() {
        console.log(this.name + " makes a noise.");
    }
}

class Dog extends Animal {
    speak() {
        console.log(this.name + " barks.");
    }
}

let dog = new Dog("Buddy");
dog.speak();

// Асинхронный код
async function fetchData() {
    try {
        let response = await fetch('https://jsonplaceholder.typicode.com/posts/1');
        let data = await response.json();
        console.log("Fetched data:", data);
    } catch (error) {
        console.error("Error fetching data:", error);
    }
}

fetchData();
