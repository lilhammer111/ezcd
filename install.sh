#!/bin/bash
set -e
## 编译项目
#cargo build --release

# 定义ezcd function目标文件路径
ezcd_func_target_dir="$HOME/.bashrc.bak.ezcd"
bashrc_path="$HOME/.bashrc"

# 定义echo输出的前缀emoji
suc_prefix_emoji="💖"
fail_prefix_emoji="💔"

# 定义输出颜色
GREEN=$'\e[0;32m'

# 检查并创建备份
if [ ! -f "$ezcd_func_target_dir" ]; then
    cp "$HOME/.bashrc" "$ezcd_func_target_dir" && echo "$suc_prefix_emoji ${GREEN}Backup created successfully." || echo "$fail_prefix_emoji Failed to create backup."
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
    echo "$suc_prefix_emoji ${GREEN}Created configuration directory at $config_dir"
fi

# 创建别名文件，如果它还不存在
if [ ! -f "$alias_file" ]; then
    touch "$alias_file"
    echo "$suc_prefix_emoji ${GREEN}Created alias file at $alias_file"
fi


# 定义ezcd函数
# shellcheck disable=SC2016
ezcd_function='
# ezcd is a shell function which utilize the ezcd-bin to make you change directory conveniently.
function ezcd() {
    # Check if no arguments are provided
    if [[ $# -eq 0 ]]; then
        echo "EZCD Error: The Arg of ezcd can not be empty."
        return
    fi

    if [[ "$1" != "--set" && "$1" != "--list" && "$1" != "--remove" && "$1" != "--help" && "$1" != "--update" ]]; then
        local dir=$(ezcd-bin "$@")
        echo "[SHELL DEBUG] The target directory is $dir."
        echo "[SHELL DEBUG] The dir is #$dir#."
        if [[ -n "$dir" && -d "$dir" ]]; then
            cd "$dir" || return
        else
            echo "$dir"
        fi
    else
        ezcd-bin "$@"
    fi
}

# 补全函数
_ezcd_completion() {
    # COMPREPLY 是 Bash 补全的数组变量，用于存储补全结果
    COMPREPLY=()
    # 当前的单词
    local current_word="${COMP_WORDS[COMP_CWORD]}"

    # 这里你可以调用你的二进制文件或其他命令来生成补全建议
    # 假设 ezcd-bin --suggest 可以根据当前输入给出建议
    local suggestions=$(ezcd-bin --suggest "$current_word")

    # 将建议结果分配给 COMPREPLY
    COMPREPLY=($(compgen -W "${suggestions}" -- "${current_word}"))
}

# 注册补全函数到 ezcd 命令
complete -F _ezcd_completion ezcd
'

# 添加ezcd函数到.bashrc，如果还没有添加的话
if ! grep -q "function ezcd()" "$bashrc_path"; then
    echo "$ezcd_function" >> "$bashrc_path"
    echo "$suc_prefix_emoji ${GREEN}The function of ezcd was added to '$bashrc_path'."
fi

# 安装二进制文件
copied_path="$ezcd_bin_dir/ezcd-bin"
cp target/release/ezcd-bin $copied_path
# shellcheck disable=SC2181
if [ $? -ne 0 ]; then
    echo "$fail_prefix_emoji Failed to copy ezcd-bin to $copied_path."
    exit 1
fi

chmod +x $copied_path
# shellcheck disable=SC2181
if [ $? -ne 0 ]; then
    echo "$fail_prefix_emoji Failed to make ezcd-bin executable."
    exit 1
fi

echo "$suc_prefix_emoji ${GREEN}The CLI tool 'ezcd' installed successfully."
echo "$suc_prefix_emoji ${GREEN}Please restart your terminal or source your '$bashrc_path' to use ezcd."
