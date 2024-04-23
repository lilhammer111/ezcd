# 🐻 Introduction

`ezcd` is a cutting-edge tool designed to revolutionize the way users navigate directories in the command line. With this tool, you can navigate subdirectories using **spaces** instead of **slashes**, and employ path **aliases** to directly access desired directories from any location. This Rust-based tool integrates seamlessly with your existing shell environment, enhancing your productivity and simplifying directory navigation. Start enhancing your command line experience with `ezcd` today.

# 🐻 Todo

- [x] Auto-complete directory names.
- [ ] Directory names are not case-sensitive.

# 🐻 Install

### Cloning

Open your terminal, enter any directory where you want to install the project, and type:

```bash
git clone https://github.com/lilhammer111/ezcd.git
```

### Compiling

Enter the project root directory:

```bash
cd ezcd
```

Compile the project, which will generate an executable named `ezcd-bin` in the `ezcd/target/release` directory:

```bash
cargo build --release
```

### Configuration

Modify the permissions of `install.sh` in the `ezcd` project root directory and execute this script:

```bash
chmod +x install.sh && ./install.sh
```

If you see an output like this, everything is successfully completed:

```bash
...
💖 The cli tool 'ezcd' installed successfully.
💖 Please restart your terminal or source your '/home/<your-name>/.bashrc' to use ezcd.
```

Okay, let's open a new terminal and start experiencing `ezcd`!

# 🐻 Usage

You can use `--help` to view all currently available commands:

```bash
ezcd --help
```

The simplest usage of `ezcd` is the same as `cd`, except that it uses **spaces** instead of **slash** symbols.

For example, in the `cd` command, if we want to enter a folder, the command might look like this:

```bash
cd Codes/RustProjects/ezcd
```

In `ezcd`, the same command would look like this:

```bash
ezcd Codes RustProjects ezcd
```

`ezcd` also has other very useful commands, for example, we can use `ezcd` to set an **alias** for the current directory. Assuming that our current directory is at `~/Codes/RustProjects/ezcd`, then we can use `--set` to set an alias for this directory:

```bash
ezcd --set ep
```

`ep` is an abbreviation for the meaning of **ezcd project**, but you can replace it with any name you like. Let's use `--list` to verify if the alias was set successfully:

```bash
ezcd --list
```

Furthermore, let's verify it practically by using the `cd` command to enter the home directory, then using `ezcd ep` to see if it switches the working directory correctly!

Well, that's about all the functionality `ezcd` supports right now. Although it's simple, we hope to enhance the capabilities of this tool in future iterations.

------

Exciting news! I've added an auto-completion feature to `ezcd`. Take a peek at the demo video below to see it in action.

**Quick Tip**: Simply hit the `Tab` key for auto-completion—unlike the usual `cd` command. Got more than one option? No problem! Just keep pressing `Tab` to cycle through all available choices. Easy, right?

![auto completion demo](/home/lilhammer/Pictures/Screenshots/auto_com_demo.gif)