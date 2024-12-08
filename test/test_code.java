public class Main {
    public static void main(String[] args) {
        // Переменные и типы данных
        int number = 10;
        double pi = 3.14;
        String message = "Hello, World!";

        // Условные операторы
        if (number > 5) {
            System.out.println("Number is greater than 5");
        } else {
            System.out.println("Number is 5 or less");
        }

        // Циклы
        for (int i = 0; i < 5; i++) {
            System.out.println("For loop iteration: " + i);
        }

        int j = 0;
        while (j < 5) {
            System.out.println("While loop iteration: " + j);
            j++;
        }

        int k = 0;
        do {
            System.out.println("Do-while loop iteration: " + k);
            k++;
        } while (k < 5);

        // Массивы
        int[] numbers = {1, 2, 3, 4, 5};
        for (int num : numbers) {
            System.out.println("Array element: " + num);
        }

        // Методы
        greet("Alice");

        // Классы и объекты
        Person person = new Person("Bob", 30);
        person.displayInfo();
    }

    // Метод
    public static void greet(String name) {
        System.out.println("Hello, " + name + "!");
    }
}

// Класс
class Person {
    private String name;
    private int age;

    // Конструктор
    public Person(String name, int age) {
        this.name = name;
        this.age = age;
    }

    // Метод
    public void displayInfo() {
        System.out.println("Name: " + name + ", Age: " + age);
    }
}

