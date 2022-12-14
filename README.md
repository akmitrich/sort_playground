# Домашнее задание №6. Простые сортировки
Оформлено в виде части Rust-библиотеки.
Данные для автоматического тестирования к проекту не прилагаются. Их необходимо получить отдельно и разместить над папкой с проектом в файловой системе.
Результаты исполнения реализованных алгоритмов приведены в таблице, которая доступна по ссылке https://docs.google.com/spreadsheets/d/1PUqMXOOZDnhlYkWCtSfcQRRo_KJqAwxsdUJ8ozCI6LY

## Bubble sort
Реализовано два варианта сортировки пузырьком. Реализация находится в модуле bubble.rs.
Для запуска примера необходимо в терминале выполнить команду cargo run --example bubble --release.
Для запуска примера с автоматизированным тестированием необходимо разместить в файловой системе папку sorting-tests над папкой с проектом и выполнить в терминале команду cargo run --example test_bubble --release

## Insertion sort
Реализовано три варианта сортировки вставкой. Реализация находится в модуле insertion.rs.
Для запуска примера необходимо в терминале выполнить команду cargo run --example insertion --release.
Для запуска примера с автоматизированным тестированием необходимо разместить в файловой системе папку sorting-tests над папкой с проектом и выполнить в терминале команду cargo run --example test_ins --release

## Shell sort
Реализовано три варианта сортировки Shell. Реализация находится в модуле shell.rs.
Для запуска примера необходимо в терминале выполнить команду cargo run --example shell --release.
Для запуска примера с автоматизированным тестированием необходимо разместить в файловой системе папку sorting-tests над папкой с проектом и выполнить в терминале команду cargo run --example test_shell --release

# Домашнее задание №7. Пирамидальная сортировка
Результаты исполнения реализованных алгоритмов приведены в таблице, которая доступна по ссылке https://docs.google.com/spreadsheets/d/1bfKErh3d18CWgHCc5-Pu7Xys2KhadYl3oUOr6geBsow

## Selection sort
Алгоритм реализован в модуле selection.rs.
Для запуска примера с автоматизированным тестированием необходимо разместить в файловой системе папку sorting-tests над папкой с проектом и выполнить в терминале команду cargo run --example test_selection --release.
Существенной разницы с простыми сортировками не обнаружено.

## Heap sort
Алгоритм реализован в модуле heapsort.rs.
Для запуска примера с автоматизированным тестированием необходимо разместить в файловой системе папку sorting-tests над папкой с проектом и выполнить в терминале команду cargo run --example test_heapsort --release.
Ускорение алгоритма явно видно при сравнении с сортировкой выбором.

# Домашнее задание №8. QuickSort, MergeSort, ExternalSort
Результаты исполнения алгоритмов QuickSort и MergeSort приведены в таблице, которая доступна по ссылке https://docs.google.com/spreadsheets/d/1bfKErh3d18CWgHCc5-Pu7Xys2KhadYl3oUOr6geBsow/edit#gid=0

## Quick sort
Алгоритм реализован в модуле quick.rs.
Для запуска примера с автоматизированным тестированием необходимо разместить в файловой системе папку sorting-tests над папкой с проектом и выполнить в терминале команду cargo run --example test_quick --release.

## Merge sort
Алгоритм реализован в модуле merge.rs.
Для запуска примера с автоматизированным тестированием необходимо разместить в файловой системе папку sorting-tests над папкой с проектом и выполнить в терминале команду cargo run --example test_merge --release.

## Внешняя сортировка
Алгоритм реализован в модуле external.
Для использования этого алгоритма предложен пример external_sort.rs. Программа управляется с помощью файла переменных окружения .env.
BASEPATH=путь к папке, в которой хранится входной файл. Эта папка будет использована для размещения временных файлов
METHOD=метод сортировки (реализованы naive и twofiles)
INPUT=имя входного файла
OUTPUT=имя выходного файла
CHUNKSIZE=размер одного блока чисел (количество i64 в одном блоке сортировки)

Для подготовки файлов со случайными числами модуль имеет функцию create_file_random_numbers.

# Домашнее задание №9. Линейные сортировки
Алгоритмы реализованы в модуле linear.
Для запуска примера с автоматизированным тестированием необходимо разместить в файловой системе папку sorting-tests над папкой с проектом и выполнить в терминале команду cargo run --example test_linear --release.