#!/bin/bash

help() {
    echo "Usage: $0 <input_video> <output_folder>"
    echo "Converts a video to a sequence of images."
    exit 1
}

# Install ffmpeg if not already installed
if ! [ -x "$(command -v ffmpeg)" ]; then
    echo "Error: ffmpeg is not installed." >&2
    # Prompt user to install ffmpeg
    echo "Install ffmpeg? (y/n)"
    read response
    if [ "$response" == "y" ]; then
        if [ -x "$(command -v apt-get)" ]; then
            sudo apt-get update
            sudo apt-get install ffmpeg
        elif [ -x "$(command -v dnf)" ]; then
            sudo dnf update
            sudo dnf install ffmpeg
        else
            echo "Error: Could not install ffmpeg. Please install manually."
            exit 1
        fi
    else
        exit 1
    fi
fi

# Check if input video is provided
if [ -z "$1" ]; then
    echo "Error: No input video provided."
    help
fi

# Create output folder if not already created
if [ -z "$2" ]; then
    echo "Error: No output folder provided."
    help
else
    mkdir -p $2
fi

# Apply filter to crop onto square and resize to 480x480

ffmpeg -i $1 -vf "crop=480:480:120:0" $2/image-%04d.bmp
