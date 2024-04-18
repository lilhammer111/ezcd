# ezcd

ezcd is a cutting-edge tool designed to revolutionize the way users navigate directories in the command line. With this tool, you can navigate subdirectories using spaces instead of slashes, and employ path aliases to directly access desired directories from any location. This Rust-based tool integrates seamlessly with your existing shell environment, enhancing your productivity and simplifying directory navigation. Start enhancing your command line experience with ezcd today.


# Usage
1. 使用`cargo build --release`编译项目，会在`ezcd/target/release`下生成一个`ezcd-bin`的可执行文件。

2. 修改ezcd项目根目录下的`install.sh`的权限并执行这个脚本，在ezcd根目录下执行：

   ```bash
    chmod +x install.sh && ./install.sh
   ```
3. 然后打开一个新终端，我们就可以开始体验`ezcd`命令了：
    
    例如，我们使用`ezcd`来为当前的ezcd项目目录设置一个别名吧。
    
    ```bash
    ezcd --set ep
   ```
   ep是ezcd project的缩写，你也可以替换成任意你自己喜欢的名称。

    让我们来验证是否成功设置了别名。

    我们可以使用`ezcd --list`列出所有别名，此外，我们来实际验证一下，通过`cd`命令进入家目录，然后使用`ezcd ep`，看看是否正确切换了工作目录！