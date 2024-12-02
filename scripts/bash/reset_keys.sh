#!/bin/bash

# Запуск скрипта
# ./reset_keys.sh <redis-client-url>

# Параметры:
# 1. redis-client-url - url Redis клиента.

: '
# Пример запуска
Первый вариант:
./reset_keys.sh localhost:8080
'

if [ -z "$1" ]; then
  echo "Использование: $0 <REDIS_CLIENT_URL>"
  exit 1
fi

LOG_FILE_NAME="reset_keys_logs.txt"
RESULTS_PATH="/results/"
KEYS_PATH="/resources/reset_keys.txt"
script_dir=$(dirname $(realpath $0))

REDIS_CLIENT_URL="$1"
readarray -t KEYS < "$script_dir$KEYS_PATH"

if [ ${#KEYS[@]} -gt 0 ]; then
  echo "Ключи для удаления данных:"
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
    curl -X DELETE "http://$REDIS_CLIENT_URL/resetKey?key=$key" \
         -m "500" \
         -s \
         -k >> $script_dir$RESULTS_PATH$LOG_FILE_NAME

    echo "" >> $script_dir$RESULTS_PATH$LOG_FILE_NAME
done
