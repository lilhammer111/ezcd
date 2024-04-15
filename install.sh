#!/bin/bash
set -e
# 编译项目
cargo build --release

# 定义ezcd function目标文件路径
ezcd_func_target_dir="$HOME/.bashrc.bak.ezcd"
bashrc_path="$HOME/.bashrc"

# 检查并创建备份
if [ ! -f "$ezcd_func_target_dir" ]; then
    cp "$HOME/.bashrc" "$ezcd_func_target_dir" && echo "Backup created successfully." || echo "Failed to create backup."
fi

# ezcd-bin目录
ezcd_bin_dir="$HOME/.local/bin"

# 定义配置目录和文件
config_dir="$HOME/.config/ezcd"
alias_file="$config_dir/aliases.list"

# 换行
echo

# 创建配置目录，如果它还不存在
if [ ! -d "$config_dir" ]; then
    mkdir -p "$config_dir"
    echo "Created configuration directory at $config_dir"
fi

# 创建别名文件，如果它还不存在
if [ ! -f "$alias_file" ]; then
    touch "$alias_file"
    echo "Created alias file at $alias_file"
fi


# 定义ezcd函数
# shellcheck disable=SC2016
ezcd_function='
# ezcd is a shell function which utilize the ezcd-bin to make you change directory conveniently.
function ezcd() {
    if [[ "$1" != "--alias" && "$1" != "--list" && "$1" != "--remove" && "$1" != "--help" && "$1" != "--update"]]; then
        local dir=$(ezcd-bin "$@")
        echo "The target directory is $dir."
        if [[ -n "$dir" && -d "$dir" ]]; then
            cd "$dir" || return
        else
            echo "The directory does not exist."
        fi
    else
        ezcd-bin "$@"
    fi
}
'

# 添加ezcd函数到.bashrc，如果还没有添加的话
if ! grep -q "function ezcd()" "$bashrc_path"; then
    echo "$ezcd_function" >> "$bashrc_path"
    echo "The function of ezcd was added to '$bashrc_path'."
fi

# 安装二进制文件
copied_path="$ezcd_bin_dir/ezcd-bin"
cp target/release/ezcd-bin $copied_path
# shellcheck disable=SC2181
if [ $? -ne 0 ]; then
    echo "Failed to copy ezcd-bin to $copied_path."
    exit 1
fi

chmod +x $copied_path
# shellcheck disable=SC2181
if [ $? -ne 0 ]; then
    echo "Failed to make ezcd-bin executable."
    exit 1
fi

echo "The CLI tool 'ezcd' installed successfully."
echo "Please restart your terminal or source your '$bashrc_path' to use ezcd."
