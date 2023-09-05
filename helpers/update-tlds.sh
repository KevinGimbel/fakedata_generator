#!/usr/bin/env bash

TLDS=""
TLD_LIST=$(curl -s https://data.iana.org/TLD/tlds-alpha-by-domain.txt | tail -n+2)

for line in $TLD_LIST; do
  TLDS+=$( printf "\"%s\",\n" "$(echo "$line" | tr '[:upper:]' '[:lower:]')" );
done

# ${TLDS::-1} removes the last character which is a trailing comma and breaks Rust 😬
TMPL=$(cat << EOF
// Module tlds provides a list of valid TLDs taken from https://data.iana.org/TLD/tlds-alpha-by-domain.txt
// last updated $(date)
// this file is generated with the update-tlds.sh file

pub const DATA_TLDS: &str = r#"
{
  "data":[
    ${TLDS::-1}
    ]
}
"#;
EOF
)

# Overwrite the tlds.rs file
echo "$TMPL" > src/data/tlds.rs
