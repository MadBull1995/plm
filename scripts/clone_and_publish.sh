#!/bin/bash
echo "##################################################"
echo "# REPO CLONING AND PUBLISHING SCRIPT              #"
echo "##################################################"

# Check for arguments
if [ "$#" -eq 0 ]; then
    echo "[ERROR] No arguments provided."
    exit 1
fi

# Define target directory for cloning repos
TARGET_DIR="./data/seed"
echo "[INFO] Setting target directory for cloning to: $TARGET_DIR"

# Create target directory if it doesn't exist
mkdir -p $TARGET_DIR

# Initialize an array to hold excludes for the current repository
declare -a current_excludes
declare -a current_dependencies
echo "##################################################"
echo "# BEGIN REPOSITORY PROCESSING                    #"
echo "##################################################"

# Loop through each argument
for arg in "$@"; do
    echo "--------------------------------------------------"
    echo "[INFO] Processing argument: $arg"

    if [[ "$arg" == "exclude="* ]]; then
        echo "[INFO] Parsing exclude patterns for the current repository."
        # Populate the excludes array for the current repository
        current_excludes="${arg#exclude=}"
        continue
    fi

    if [[ "$arg" == "deps="* ]]; then
        echo "[INFO] Parsing dependencies for the current repository."
        # Populate the excludes array for the current repository
        current_dependencies="${arg#deps=}"
        continue
    fi

    # Split the repo and source directory using IFS
    IFS=":"
    read -ra ADDR <<< "$arg"

    # Validate if the arguments are present
    if [ -z "${ADDR[0]}" ] || [ -z "${ADDR[1]}" ] || [ -z "${ADDR[2]}" ]; then
        echo "[ERROR] Invalid argument format. Expected repo:src/:branch"
        continue
    fi

    repo="${ADDR[0]}"
    src="${ADDR[1]}"
    branch="${ADDR[2]}"

    # current_excludes=${excludes[$repo]}

    echo "[INFO] Preparing to clone repository:"
    echo "       - Repo: https://github.com/$repo.git"
    echo "       - Source Directory: ${src}"
    echo "       - Exclude Patterns: $current_excludes"

    # Initialize a new repository in the target directory
    mkdir -p "$TARGET_DIR/$repo"
    cd "$TARGET_DIR/$repo"
    
    git init

    # Add the remote and configure sparse-checkout
    git remote add origin "https://github.com/$repo.git"
    # git config core.sparseCheckout true
    # echo "$src" >> .git/info/sparse-checkout

    # Apply excludes
    # for exclude in "${current_excludes[@]}"; do
    #     echo "!$exclude" >> .git/info/sparse-checkout
    # done
    
    echo "[INFO] Fetching and checking out the repository."

    # Fetch data and checkout
    git pull origin $branch

    # Check if it's a shallow clone and unshallow it if needed
    if [ -f "$(git rev-parse --git-dir)/shallow" ]; then
        echo "[INFO] Repository is shallow. Unshallowing..."
        git fetch --unshallow
    fi

    # Fetch tags from remote
    git fetch --tags
    
    # Extract the repo name
    repo_name=$(basename "$repo")

    # Get the latest tag
    latest_tag=$(git describe --tags `git rev-list --tags --max-count=1`)

    # Checkout the latest tag
    # git checkout $latest_tag
    
    echo "[INFO] Preparing to publish the library."
    
    latest_tag=${latest_tag#v}  # Remove a leading 'v', if present

    # Strip off any suffix after "-" to conform to major.minor.patch
    formatted_version=$(echo "$latest_tag" | sed 's/-.*//')
    latest_tag=${formatted_version}  # Use the formatted version
    # Check if the variable is empty
    if [ -z "$latest_tag" ]; then
        latest_tag="0.0.1"
    fi

    # Construct the exclude string if necessary
    exclude_str=""
    if [ ${#current_excludes[@]} -ne 0 ]; then
        exclude_str="--exclude ${current_excludes[@]}"
    fi
    
    # Construct the exclude string if necessary
    deps_str=""
    if [ ${#current_dependencies[@]} -ne 0 ]; then
        deps_str="--dependencies ${current_dependencies[@]}"
    fi
    

    # Run the command on the src directory
    if [ -d "${src}" ]; then
        echo "Running plm publish on ${repo_name}..."
        # Replace 'YOUR_COMMAND_HERE' with the command you want to run
        plm -d init \
            --library-name @$repo \
            --src-dir $src \
            --description "$repo_name official library" \
            --version $latest_tag \
            --license apache2 \
            $deps_str \
            $exclude_str && plm install && plm publish --preserve-imports
        cd - > /dev/null 2>&1
    else
        echo "[ERROR] Directory ${src} does not exist in the repo ${repo_name}:${formatted_version}."
    fi
    # Clear the excludes for the next repository
    current_excludes=()
    echo "--------------------------------------------------"

    # Go back to the original directory
    cd - > /dev/null 2>&1
done


echo "##################################################"
echo "# END REPOSITORY PROCESSING                      #"
echo "##################################################"