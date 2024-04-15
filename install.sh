#!/bin/bash

# 编译项目
cargo build --release

# 检查并创建备份
#cp ~/.bashrc ~/.bashrc.backup

# 定义ezcd函数
# shellcheck disable=SC2016
ezcd_function='
# ezcd is a shell function which utilize the ezcd-bin to make you change directory conveniently.
function ezcd() {
    local dir=$(ezcd-bin "$@")
    if [[ -n "$dir" && -d "$dir" ]]; then
        cd "$dir" || return
    else
        echo "The directory does not exist: $dir"
    fi
}
'

# 添加ezcd函数到.bashrc，如果还没有添加的话
if ! grep -q "function ezcd()" ~/.bashrc; then
    echo "$ezcd_function" >> ~/.bashrc
    echo "ezcd function added to .bashrc"
fi

# 安装二进制文件
cp target/release/ezcd-bin ~/.local/bin/ezcd-bin
# shellcheck disable=SC2181
if [ $? -ne 0 ]; then
    echo "Failed to copy ezcd-bin to ~/.local/bin."
    exit 1
fi

chmod +x ~/.local/bin/ezcd-bin
# shellcheck disable=SC2181
if [ $? -ne 0 ]; then
    echo "Failed to make ezcd-bin executable."
    exit 1
fi

echo "ezcd-bin installed successfully"

echo "Installation complete. Please restart your terminal or source your .bashrc to use ezcd."

