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
    # 检查是否已经获取了补全列表
    if [ -z "${#COMPREPLY[@]}" ]; then
        local current_word="${COMP_WORDS[COMP_CWORD]}"
        local words = "${COMP_WORDS}"
        local suggestions=$(ezcd-bin --suggest "$words")
        COMPREPLY=($(compgen -W "${suggestions}" -- "${current_word}"))
        # 初始化补全索引
        COMP_CWORD_BACKUP=0
    fi

    # 检查是否是连续的 Tab 按键
    if [[ "${COMP_TYPE}" = "tab" ]]; then
        # 移动到下一个补全选项
        ((COMP_CWORD_BACKUP++))
        # 如果索引超出了补全列表的范围，则重置
        if [ "${COMP_CWORD_BACKUP}" -ge "${#COMPREPLY[@]}" ]; then
            COMP_CWORD_BACKUP=0
        fi
        COMPREPLY=("${COMPREPLY[COMP_CWORD_BACKUP]}")
    fi
}

# 使用 complete 命令注册补全函数
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
echo
