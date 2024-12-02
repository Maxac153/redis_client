import os


def delete_all_files_in_directory(directory_path):
    try:
        for filename in os.listdir(directory_path):
            file_path = os.path.join(directory_path, filename)
            if os.path.isfile(file_path):
                os.remove(file_path)
                print(f'Удален файл: {file_path}')
    except OSError as e:
        print(f'Ошибка при удалении файлов: {e}')
