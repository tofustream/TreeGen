# TreeGen

A simple directory tree generator built with [iced-rs](https://github.com/iced-rs/iced).

![TreeGen_MainForm](https://github.com/user-attachments/assets/e5e8fe06-2d53-4d90-a8c3-dffebe19610d)

## Features

- Generate a visual representation of directory trees.
- Filter directories and files before generating the tree.
- Copy the generated directory tree structure to the clipboard for easy sharing or documentation.
- Simple and intuitive GUI built using [iced-rs](https://github.com/iced-rs/iced).

## Installation

To run TreeGen, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/tofustream/TreeGen.git
    cd TreeGen
    ```

2. Install the required dependencies:

    ```bash
    cargo build
    ```

3. Run the application:

    ```bash
    cargo run
    ```

## Usage

1. **Browse for Folder**: Use the `Browse` button to select a folder for which you want to generate a directory tree.
2. **Filter the Tree**: Click on the `Filter` button to open a filter modal where you can check/uncheck files and directories to include or exclude from the generated tree.
3. **Generate Tree**: Once you have applied any filters, the directory tree will be displayed. You can scroll through the tree to view the structure.
4. **Copy to Clipboard**: Use the `Copy to Clipboard` button to copy the tree structure for easy sharing.

## Example Output

Here is an example of what the generated tree might look like:

```
Example
|-- .DS_Store
|-- ex_dir
| |__ ex1.txt
|__ ex2.txt
```
