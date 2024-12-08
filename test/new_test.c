
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Структура
struct Person {
    char name[50];
    int age;
};

// Функция для вывода информации о человеке
void printPerson(struct Person p) {
    printf("Name: %s, Age: %d\n", p.name, p.age);
}

// Функция для вычисления факториала
int factorial(int n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

// Функция для вычисления суммы элементов массива
int sumArray(int arr[], int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += arr[i];
    }
    return sum;
}

// Основная функция
int main() {
    // Объявление переменных
    int a = 10;
    float b = 3.14;
    char c = 'A';
    char str[] = "Hello, World!";

    // Условные операторы
    if (a > 5) {
        printf("a is greater than 5\n");
    } else {
        printf("a is not greater than 5\n");
    }

    // Цикл for
    for (int i = 0; i < 5; i++) {
        printf("i = %d\n", i);
    }

    // Цикл while
    int j = 0;
    while (j < 3) {
        printf("j = %d\n", j);
        j++;
    }

    // Цикл do...while
    int k = 0;
    do {
        printf("k = %d\n", k);
        k++;
    } while (k < 2);

    // Массивы
    int arr[5] = {1, 2, 3, 4, 5};
    int sum = sumArray(arr, 5);
    printf("Sum of array elements: %d\n", sum);

    // Указатели
    int *ptr = &a;
    printf("Value of a using pointer: %d\n", *ptr);

    // Структуры
    struct Person person;
    strcpy(person.name, "John Doe");
    person.age = 30;
    printPerson(person);

    // Рекурсивная функция
    int fact = factorial(5);
    printf("Factorial of 5: %d\n", fact);

    // Динамическое выделение памяти
    int *dynamicArray = (int *)malloc(5 * sizeof(int));
    if (dynamicArray == NULL) {
        printf("Memory allocation failed\n");
        return 1;
    }
    dynamicArray[0] = 10;
    dynamicArray[1] = 20;
    dynamicArray[2] = 30;
    dynamicArray[3] = 40;
    dynamicArray[4] = 50;
    printf("Dynamic array elements: %d %d %d %d %d\n", dynamicArray[0], dynamicArray[1], dynamicArray[2], dynamicArray[3], dynamicArray[4]);
    free(dynamicArray);

    // Конструкция switch-case
    int choice = 2;
    switch (choice) {
        case 1:
            printf("Choice is 1\n");
            break;
        case 2:
            printf("Choice is 2\n");
            break;
        case 3:
            printf("Choice is 3\n");
            break;
        default:
            printf("Invalid choice\n");
            break;
    }

    return 0;
}
