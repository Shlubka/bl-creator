#include <stdio.h>

// Глобальная переменная
int globalVar = 10;

// Прототипы функций
void printMessage(const char *message);
int add(int a, int b);
void testLoop();
void testSwitch();
void testArray();
void testPointer();
void testStruct();
void testComplexIfElse(int value);

int main() {
    // Локальная переменная
    int localVar = 20;

    // Вызов функции
    printMessage("Hello, World!");

    // Арифметические операции
    int sum = add(5, 3);
    printf("Sum: %d\n", sum);

    // Условные операторы
    if (sum > 5) {
        printf("Sum is greater than 5\n");
    } else {
        printf("Sum is not greater than 5\n");
    }

    // Циклы
    testLoop();

    // Оператор switch
    testSwitch();

    // Массивы
    testArray();

    // Указатели
    testPointer();

    // Структуры
    testStruct();

    // Сложная вложенность if-else
    testComplexIfElse(15);

    return 0;
}

// Определение функции
void printMessage(const char *message) {
    printf("%s\n", message);
}

int add(int a, int b) {
    return a + b;
}

void testLoop() {
    int i;
    for (i = 0; i < 5; i++) {
        printf("For loop iteration: %d\n", i);
    }

    i = 0;
    while (i < 5) {
        printf("While loop iteration: %d\n", i);
        i++;
    }

    i = 0;
    /*do {
        printf("Do-while loop iteration: %d\n", i);
        i++;
    } while (i < 5);*/
}

void testSwitch() {
    int number = 2;
    switch (number) {
        case 1:
            printf("Number is 1\n");
            break;
        case 2:
            printf("Number is 2\n");
            break;
        default:
            printf("Number is not 1 or 2\n");
    }
}

void testArray() {
    int arr[5] = {1, 2, 3, 4, 5};
    for (int i = 0; i < 5; i++) {
        printf("Array element %d: %d\n", i, arr[i]);
    }
}

void testPointer() {
    int var = 10;
    int *ptr = &var;
    printf("Value of var: %d\n", var);
    printf("Value of *ptr: %d\n", *ptr);
    printf("Address of var: %p\n", (void *)&var);
    printf("Address stored in ptr: %p\n", (void *)ptr);
}

void testStruct() {
    struct Point {
        int x;
        int y;
    };

    struct Point p1;
    p1.x = 10;
    p1.y = 20;

    printf("Point p1: (%d, %d)\n", p1.x, p1.y);
}

void testComplexIfElse(int value) {
    if (value > 10) {
        if (value % 2 == 0) {
            if (value > 20) {
                printf("Value is greater than 20 and even\n");
            } else {
                printf("Value is between 10 and 20 and even\n");
            }
        } else {
            if (value > 20) {
                printf("Value is greater than 20 and odd\n");
            } else {
                printf("Value is between 10 and 20 and odd\n");
            }
        }
    } else {
        if (value % 2 == 0) {
            if (value > 5) {
                printf("Value is between 5 and 10 and even\n");
            } else {
                printf("Value is less than 5 and even\n");
            }
        } else {
            if (value > 5) {
                printf("Value is between 5 and 10 and odd\n");
            } else {
                printf("Value is less than 5 and odd\n");
            }
        }
    }
}
