#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Person { // Структура

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
