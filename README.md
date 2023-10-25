[![Build binary release](https://github.com/nogibjj/Individual-Project-2-njs35/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/Individual-Project-2-njs35/actions/workflows/release.yml)
[![Clippy](https://github.com/nogibjj/Individual-Project-2-njs35/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/Individual-Project-2-njs35/actions/workflows/lint.yml)
[![Rustfmt](https://github.com/nogibjj/Individual-Project-2-njs35/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/nogibjj/Individual-Project-2-njs35/actions/workflows/rustfmt.yml)
[![Tests](https://github.com/nogibjj/Individual-Project-2-njs35/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/Individual-Project-2-njs35/actions/workflows/tests.yml)

# Rust SQLite CLI with Optimized Binary
## Project Overview
This project is centered around building a Rust CLI tool that allows the user to perform CRUD operations on a SQLite database. The CLI tool is straightforward to use, and takes a SQL query of any format as an input. 

While developing this project, I utilized GitHub Copilot to expedite the coding process. Copilot did a great job at suggesting error handling techniques in Rust, and I was able to learn a lot about the language by using it. Copilot also helped me to connect to and query the SQLite database using the rusqlite library. Additionally, Copilot helped me to improve the robustness of my tests to ensure that they were verifying the correct behavior of the functions.

This project also effectively handles errors in Rust. The code checks if an appropriate number of command-line arguments is provided. If not, it logs a report and returns early. This ensures that the program doesn't proceed to the querying stage with incomplete or incorrect input. The code also makes use of the Result type when calling the functions execute_read_query and execute_cud_query. In both cases, it uses a match expression to handle the potential results, either success (Ok) or failure (Err). This enforces explicit error handling and helps prevent unwarranted panics.

## CLI Tool Usage and Dependency Installation


