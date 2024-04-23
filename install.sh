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
log_dir="$HOME/.config/ezcd/log"
env_shell_file="$HOME/.config/ezcd/env.sh"

# 换行
echo

# 创建配置目录，如果它还不存在
if [ ! -d "$config_dir" ]; then
    mkdir -p "$config_dir"
    echo "$suc_prefix_emoji ${GREEN}Created configuration directory at $config_dir."
fi

# 创建日志文件夹，如果他还不存在
if [ ! -d "$log_dir" ]; then
    mkdir -p "$log_dir"
    echo "$suc_prefix_emoji ${GREEN}Created log directory at $log_dir."
fi

# 创建别名列表文件，如果它还不存在
if [ ! -f "$alias_file" ]; then
    touch "$alias_file"
    echo "$suc_prefix_emoji ${GREEN}Created alias file at $alias_file."
fi

# 创建env shell脚本文件
if [ ! -f "$env_shell_file" ]; then
    touch "$env_shell_file"
    echo "$suc_prefix_emoji ${GREEN}Created env shell file at $env_shell_file."
fi

# 定义ezcd函数
# shellcheck disable=SC2016
env_shell_content='# ezcd is a shell function which utilize the ezcd-bin to make you change directory conveniently.
function ezcd() {
    # Check if no arguments are provided
    if [[ $# -eq 0 ]]; then
        echo "EZCD Error: The Arg of ezcd can not be empty."
        return
    fi
    if [[ "$1" != "--set" && "$1" != "--list" && "$1" != "--remove" && "$1" != "--help" && "$1" != "--update" ]]; then
        local dir=$(ezcd-bin "$@")

        if [[ -n "$dir" && -d "$dir" ]]; then
            cd "$dir" || return
        else
            echo "$dir"
        fi
    else
        ezcd-bin "$@"
    fi
}

declare -a EZCD_COMPLETIONS=()
declare -i EZCD_INDEX=0
declare PREV_COMP_WORDS=""

_ezcd_completion() {
    local current_word="${COMP_WORDS[COMP_CWORD]}"
    if [[ ! " ${EZCD_COMPLETIONS[*]} " =~ " ${current_word} " ]]; then
        EZCD_COMPLETIONS=()
        EZCD_INDEX=0
        IFS=" " read -ra EZCD_COMPLETIONS <<< "$(ezcd-bin --suggest "${COMP_WORDS[@]}")"
    fi

    if [[ "${#EZCD_COMPLETIONS[@]}" -gt 1 ]]; then
        if [[ "$EZCD_INDEX" -ge "${#EZCD_COMPLETIONS[@]}" ]]; then
            EZCD_INDEX=0
        fi
        COMPREPLY=("${EZCD_COMPLETIONS[EZCD_INDEX]}")
        ((EZCD_INDEX++))
    elif [[ "${#EZCD_COMPLETIONS[@]}" -eq 1 ]]; then
        COMPREPLY=("${EZCD_COMPLETIONS[0]} ")
    else
        COMPREPLY=()
    fi
}

complete -o nospace -F _ezcd_completion ezcd
'

# 覆盖写入上述脚本到env_shell_file中
echo "$env_shell_content" > "$env_shell_file"
if [ $? -eq 0 ]; then
    echo "$suc_prefix_emoji ${GREEN}Ezcd configuration has been successfully written to $env_shell_file."
else
    echo "$fail_prefix_emoji Failed to write ezcd configuration to $env_shell_file."
fi

# 检查是否已经有了这个 source 行
if grep -Fxq "source $env_shell_file" "$bashrc_path"
then
    echo "$suc_prefix_emoji ${GREEN}The source line already exists in $bashrc_path."
else
    echo "source $env_shell_file" >> "$bashrc_path"
    echo "$suc_prefix_emoji ${GREEN}Added 'source $env_shell_file' to $bashrc_path."
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
echo
