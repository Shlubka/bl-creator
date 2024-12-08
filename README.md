# How it works
это программа компилирует входной код в json или XML (по выбору). сейчас поддерживается только rust, скоро с. в будущем планируется расширение до всех популярных в снг языков. Блок-схема создания в соответствии с ГОСТом
# How to use it
 Команда запуска `json-compiler --<language> --<path to code>`
 Этот файл надо загрузить на следующий сервисы: 
- если выбрали [json](https://programforyou.ru/block-diagram-redactor)
- если выбрали [xml](https://app.diagrams.net/)
# Todo
- [ ] xml
- [x] анализ с учётом неизвестной глубины вложенности
- [x] переход на [tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- [x] swich-case (без привязки к языку)
- [ ] ==другие языки==
- [ ] selenium
