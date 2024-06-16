#!/bin/bash

export PATH=/home/ec2-user/.nvm/versions/node/v20.11.1/bin:/usr/local/sbin:/usr/local/bin:/sbin:/bin:/usr/sbin:/usr/bin:/opt/bin

# List of channels
keys=("Unexpected")

times_Unexpected=("06:05" "08:45" "13:15" "15:20" "18:35" "21:55")

current_time=$(date +"%H:%M")

for i in "${!keys[@]}"; do
    key=${keys[$i]}
    varname="times_$key"

    eval times=(\"\${${varname}[@]}\")

    for time in "${times[@]}"; do
        if [[ "$current_time" == "$time" ]]; then
            echo "EXECUTING for $key at $time"
            cd /home/ec2-user/rust_utils
            ./target/debug/news_scraper $key
            break 2
        fi
    done
done
