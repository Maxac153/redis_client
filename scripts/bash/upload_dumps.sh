#!/bin/bash

# Запуск скрипта
# ./upload_dumps.sh <redis-client-url>

# Параметры:
# 1. redis-client-url - url Redis клиента.

: '
# Пример запуска
./upload_dumps.sh localhost:8080
'

if [ -z "$1" ]; then
  echo "Использование: $0 <REDIS_CLIENT_URL>"
  exit 1
fi

RESULTS_PATH="/results/"
LOG_FILE_NAME="upload_dumps_logs.txt"
KEYS_PATH="/resources/upload_keys.txt"
script_dir=$(dirname $(realpath $0))

REDIS_CLIENT_URL="$1"
readarray -t KEYS < "$script_dir$KEYS_PATH"

if [ ${#KEYS[@]} -gt 0 ]; then
  echo "Ключи для загрузки дампа:"
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
    curl -X POST "http://$REDIS_CLIENT_URL/uploadDumpKey?key_name=$key" \
         --header "Content-Type: application/octet-stream" \
         --data-binary "@$script_dir$RESULTS_PATH/dumps/$key.dump" \
         -m "500" \
         -s \
         -k >> $script_dir$RESULTS_PATH$LOG_FILE_NAME

    echo "" >> $script_dir$RESULTS_PATH$LOG_FILE_NAME
done
