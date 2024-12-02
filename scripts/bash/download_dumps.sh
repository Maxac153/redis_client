#!/bin/bash

# Запуск скрипта
# ./download_dumps.sh <redis-client-url>

# Параметры:
# 1. redis-client-url - url Redis клиента.

: '
# Пример запуска
./download_dumps.sh localhost:8080
'

if [ -z "$1" ]; then
  echo "Использование: $0 <REDIS_CLIENT_URL>"
  exit 1
fi

RESULTS_PATH="/results/"
LOG_FILE_NAME="download_dumps_logs.txt"
KEYS_PATH="/resources/download_keys.txt"
script_dir=$(dirname $(realpath $0))

REDIS_CLIENT_URL="$1"
readarray -t KEYS < "$script_dir$KEYS_PATH"

if [ ${#KEYS[@]} -gt 0 ]; then
  echo "Ключи для скачивания дампа:"
  for key in "${KEYS[@]}"; do
    echo "- $key"
  done
else
  echo "Использование: $0 <REDIS_CLIENT_URL>"
  echo "Не указан массив ключей!"
  exit 1
fi

if [ -f "$script_dir$RESULTS_PATH$LOG_FILE_NAME" ]; then
  rm "$script_dir$RESULTS_PATH$LOG_FILE_NAME"
else
  echo "Файл $LOG_FILE_NAME не найден."
fi

for key in "${KEYS[@]}"; do
    curl -X GET "http://$REDIS_CLIENT_URL/downloadDumpKey?key=$key" \
         -o $script_dir$RESULTS_PATH/dumps/"$key".dump \
         -o /dev/null \
         -w "%{http_code}" \
         -m "500" \
         -s \
         -k >> $script_dir$RESULTS_PATH$LOG_FILE_NAME

    echo " - $key" >> $script_dir$RESULTS_PATH$LOG_FILE_NAME
done
