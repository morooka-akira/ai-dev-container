#\!/bin/bash
awl() {
    local target_path
    target_path=$(cargo run -- list)
    if [ -n "$target_path" ]; then
        echo "Would navigate to: $target_path"
        # cd "$target_path"
    else
        echo "No workspace selected"
    fi
}

# Test the function
awl
