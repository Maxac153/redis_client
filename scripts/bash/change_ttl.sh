#!/bin/bash

# Запуск скрипта
# ./change_ttl.sh <redis-client-url> <ttl> (sec)

# Параметры:
# 1. redis-client-url - url Redis клиента;
# 2. ttl - время жизни ключей (sec).

: '
# Пример запуска
./change_ttl.sh localhost:8080 86400
'

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Использование: $0 <REDIS_CLIENT_URL> <TTL> (sec)"
  exit 1
fi

RESULTS_PATH="/results/"
LOG_FILE_NAME="change_ttl_logs.txt"
KEYS_PATH="/resources/change_ttl_keys.txt"
script_dir=$(dirname $(realpath $0))

REDIS_CLIENT_URL="$1"
TTL="$2"
readarray -t KEYS < "$script_dir$KEYS_PATH"

if [ ${#KEYS[@]} -gt 0 ]; then
  echo "Ключи для изменения TTL:"
  for key in "${KEYS[@]}"; do
    echo "- $key"
  done
else
  echo "Использование: $0 <REDIS_CLIENT_URL> <TTL> (sec)"
  echo "Не указан массив ключей!"
  exit 1
fi

if [ -f "$script_dir$RESULTS_PATH$LOG_FILE_NAME" ]; then
  rm "$script_dir$RESULTS_PATH$LOG_FILE_NAME"
else
  echo "Файл $LOG_FILE_NAME не найден."
fi

for key in "${KEYS[@]}"; do
    curl -X PATCH "http://$REDIS_CLIENT_URL/changeTtl?ttl=$TTL&key=$key" \
         -m "500" \
         -s \
         -k >> $script_dir$RESULTS_PATH$LOG_FILE_NAME

    echo "" >> $script_dir$RESULTS_PATH$LOG_FILE_NAME
done
