#!/bin/bash

if [ -z "$1" ] || [ -z "$2" ]; then
    echo "使用方法: ./run.sh [言語] [フォルダ/ファイル名]"
    echo "例（JS）  : ./run.sh js tessoku/001"
    echo "例（Java）: ./run.sh java tessoku/A01"
    echo "例（Rust）: ./run.sh rs tessoku/001"
    exit 1
fi

LANG=$1
FILE=$2

if [ "$LANG" = "rs" ]; then
    if [[ "$FILE" != *\.rs ]]; then
        FILE="${FILE}.rs"
    fi
    
    echo "実行中: Rust (rs/$FILE)"
    if [ ! -f "rs/$FILE" ]; then
        echo "エラー: rs/$FILE が見つかりません。ファイルを作成してください。"
        exit 1
    fi

    docker compose exec rs sh -c "cp \"$FILE\" src/main.rs && cargo run"

elif [ "$LANG" = "java" ]; then
    if [[ "$FILE" != *\.java ]]; then
        FILE="${FILE}.java"
    fi
    
    echo "実行中: Java (java/$FILE)"
    docker compose exec java java "$FILE"

elif [ "$LANG" = "js" ]; then
    if [[ "$FILE" != *\.js ]]; then
        FILE="${FILE}.js"
    fi
    
    echo "実行中: JavaScript (js/$FILE)"
    docker compose exec js node "$FILE"

else
    echo "エラー: 対応していない言語です。rs, java, js のいずれかを指定してください。"
fi

# ./run.sh rs abs/001
# ./run.sh java abs/001
# ./run.sh js abs/001