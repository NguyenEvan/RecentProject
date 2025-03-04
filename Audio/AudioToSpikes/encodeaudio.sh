#!/bin/bash


DIRECTORY="dataset/Audio/AudioFiles/Spoken_Phrases"
OUTPUT_DIRECTORY="dataset/Audio/AudioFiles/Spike_Trains"
NUM_CHANNELS=70
VERBOSE="--verbose"
REPO_URL="https://github.com/electronicvisions/lauscher.git"
REPO_DIR="lauscher"
VENV_DIR="venv"

# Clone the repository if it doesn't exist
if [ ! -d "$REPO_DIR" ]; then
    echo "Cloning the repository..."
    git clone "$REPO_URL"
    cd "$REPO_DIR"
else
    cd "$REPO_DIR"
fi

# Set up virtual environment if it doesn't exist
if [ ! -d "$VENV_DIR" ]; then
    echo "Setting up virtual environment..."
    python3 -m venv "$VENV_DIR"
fi

# Activate virtual environment
source "$VENV_DIR/bin/activate"

# Install dependencies
pip install -r requirements.txt


for FLAC_FILE in "$DIRECTORY"/*.flac; do
    BASE_NAME=$(basename "$FLAC_FILE" .flac)
    NPZ_FILE="$../$OUTPUT_DIRECTORY/{BASE_NAME}.npz"
    
    python -m lauscher "$FLAC_FILE" "$NPZ_FILE" --num_channels "$NUM_CHANNELS" $VERBOSE
    
    # Check if the encoding was successful
    if [ $? -eq 0 ]; then
        echo "Successfully encoded $FLAC_FILE to $NPZ_FILE"
    else
        echo "Failed to encode $FLAC_FILE" >&2
    fi
done
