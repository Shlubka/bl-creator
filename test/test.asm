section .data
    msg db 'Hello, World!', 0
    num1 dd 10
    num2 dd 20
    result dd 0

section .bss
    buffer resb 256

section .text
    global _start

_start:
    ; Вывод сообщения
    mov eax, 4          ; Системный вызов для записи (sys_write)
    mov ebx, 1          ; Файловый дескриптор (stdout)
    mov ecx, msg        ; Адрес сообщения
    mov edx, 13         ; Длина сообщения
    int 0x80            ; Вызов ядра

    ; Арифметические операции
    mov eax, [num1]     ; Загрузка num1 в eax
    add eax, [num2]     ; Сложение num2 к eax
    mov [result], eax   ; Сохранение результата в result

    ; Условные операторы
    cmp eax, 30         ; Сравнение eax с 30
    jg greater_than_30   ; Переход, если eax > 30
    jl less_than_30     ; Переход, если eax < 30
    je equal_to_30      ; Переход, если eax == 30

greater_than_30:
    mov eax, 4          ; Системный вызов для записи (sys_write)
    mov ebx, 1          ; Файловый дескриптор (stdout)
    mov ecx, msg        ; Адрес сообщения
    mov edx, 13         ; Длина сообщения
    int 0x80            ; Вызов ядра
    jmp end_program     ; Переход к концу программы

less_than_30:
    mov eax, 4          ; Системный вызов для записи (sys_write)
    mov ebx, 1          ; Файловый дескриптор (stdout)
    mov ecx, msg        ; Адрес сообщения
    mov edx, 13         ; Длина сообщения
    int 0x80            ; Вызов ядра
    jmp end_program     ; Переход к концу программы

equal_to_30:
    mov eax, 4          ; Системный вызов для записи (sys_write)
    mov ebx, 1          ; Файловый дескриптор (stdout)
    mov ecx, msg        ; Адрес сообщения
    mov edx, 13         ; Длина сообщения
    int 0x80            ; Вызов ядра
    jmp end_program     ; Переход к концу программы

end_program:
    ; Завершение программы
    mov eax, 1          ; Системный вызов для выхода (sys_exit)
    xor ebx, ebx        ; Код возврата 0
    int 0x80            ; Вызов ядра
