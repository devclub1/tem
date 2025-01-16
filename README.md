# tem

#### Clone your templates *blazingly* fast

## Content
- [1. Installation](#1-installation)
- [2. Usage](#2-usage)
- [3. Notes](#3-notes)

#
### 1. Installation
- Install `tem` using one of the following methods:
    - **Using Cargo** (recommended if `rustup` is installed or `cargo bin` is in your `PATH`):
        ```bash
        cargo install tem
        ```
    - **Building from source**:
        1. Clone the repository:
            ```bash
            git clone <repository-url>
            ```
        2. Build the program:
            ```bash
            cd tem
            cargo build --release
            ```
        3. Use the compiled binary in `./target/release/tem`
    - **Using a prebuilt binary**:
        1. Download the attached binary for your platform from the release page
        2. Run the binary directly:
            ```bash
            ./tem
            ```

#
### 2. Usage
1. Initialize the configuration:
    ```bash
    tem init
    ```
   This will create a configuration file at the default location written in the output (e.g., `~/.config/.tem/config.toml`)

2. Modify the generated configuration file to include your own templates
    ```toml
    [git]
    react-vite = ["git@github.com:axbg/react-vite-starter"] # example template
    ```

3. Generate a project using a template:
    ```bash
    tem <<template_name>> <<project_name>>
    ```
   Replace `<<template_name>>` with the desired template name and `<<project_name>>` with the name of the target directory

4. Use the --help flag to learn more about additional features, like branch selection and implicit cloning
    ```bash
    tem --help
    ```

#
### 3. Notes
- Supported processors:
    - **git**
    - *more processors will be added in the future*
