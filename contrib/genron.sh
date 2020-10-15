#!/bin/bash

# Simple script to generate a starting point for a VolcaSample ron file
#
# Usage:
#
# ls dir_with_samples | genron.sh > dir_with_samples/samples.ron
#

set -euo pipefail
IFS=$'\n\t'

cat <<EOT
#![enable(implicit_some)]
VolcaSample(
    samples: {
EOT

index=0

while read entry; do
    # ignore .ron file
    if [[ ! $entry =~ ^.*.ron$ ]]; then
        cat <<EOT
            ${index}: Sample((
                file: "${entry}",
            )),
EOT
        index=$((index+1))
    fi
done

cat <<EOT
    },
)
EOT