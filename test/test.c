
#include <stdio.h>

// Объявление функции

void printMessage(const char *name) {
    printf("Привет, %s!\n", name);
}

int main() {
    // Объявление переменных
    int a, b, sum;
    char name[50];

    // Ввод данных
    printf("Введите ваше имя: ");
    scanf("%s", name);

    printf("Введите два целых числа: ");
    scanf("%d %d", &a, &b);

    // Условные операторы с вложенностью
    if (a > b) {
        printf("%d больше %d\n", a, b);
        if (a > 10) {
            printf("%d также больше 10\n", a);
        } else {
            printf("%d меньше или равно 10\n", a);
        }
    } 
    else if (a < b) {
        printf("%d меньше %d\n", a, b);
        if (b > 10) {
            printf("%d также больше 10\n", b);
        } else {
            printf("%d меньше или равно 10\n", b);
        }
    } 
    else {
        printf("%d равно %d\n", a, b);
    }

    // Цикл for
    printf("Цикл for: ");
    for (int i = 0; i < 5; i++) {
        printf("%d ", i);
    }
    printf("\n");

    // Цикл while
    int i = 0;
    printf("Цикл while: ");
    while (i < 5) {
        printf("%d ", i);
        i++;
    }
    printf("\n");

    // Цикл do...while
    /*i = 0;
    printf("Цикл do...while: ");
    do {
        printf("%d ", i);
        i++;
    } while (i < 5);
    printf("\n");
*/

    // Выполнение арифметической операции
    sum = a + b;
    printf("Сумма %d и %d равна %d\n", a, b, sum);

    // Вызов функции
    printMessage(name);

    return 0;
}

// Определение функции
