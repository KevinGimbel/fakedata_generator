#!/usr/bin/env bash

FILE="$1"
KEY="$2"
NAME="$3"

if [[ -z "$KEY" ]]; then
    FILE_BASE="${FILE##*/}"
    KEY="$(echo -n "${FILE_BASE%.*}")"
fi

if [[ -z "$NAME" ]]; then
    FILE_BASE="${FILE##*/}"
    NAME="$(echo -n "${FILE_BASE%.*}" | tr '[:lower:]' '[:upper:]')"
fi

cat <<EOF >> src/data/corpora.rs
pub const DATA_${NAME}: &str = r#"
{
  "data": $(cat $FILE | jq .$KEY)
}"#;

EOF
