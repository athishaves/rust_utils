#!/bin/bash

export PATH=/home/ec2-user/.nvm/versions/node/v20.11.1/bin:/usr/local/sbin:/usr/local/bin:/sbin:/bin:/usr/sbin:/usr/bin:/opt/bin

# List of channels
keys=("unexpected")

times_unexpected=("06:05" "06:45" "08:45" "10:15" "13:10" "15:20" "16:30" "19:05" "20:20" "21:55" "22:30" "23:35")

current_time=$(date +"%H:%M")

for i in "${!keys[@]}"; do
    key=${keys[$i]}
    varname="times_$key"
    env_file="$key.env"

    eval times=(\"\${${varname}[@]}\")

    for time in "${times[@]}"; do
        if [[ "$current_time" == "$time" ]]; then
            echo "EXECUTING for $key at $time"
            cd /home/ec2-user/rust_utils
            ./target/debug/news_scraper $env_file
            break 2
        fi
    done
done
