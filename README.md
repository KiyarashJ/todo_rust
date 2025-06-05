# 🌟 Todo CLI: Your Colorful Task Manager! 🚀

Welcome to **Todo CLI**! 🎉 A simple, lightweight command-line todo list application built with 🦀 **Rust**. It lets you add tasks, mark them as done or not, attach timestamps using `chrono`, and save everything as JSON using `serde_json`. Plus, it appends tasks to a file without overwriting your precious todos! 📝💾

---

## 🌈 Features

- **Add Tasks**: Enter your todos in the command prompt. 📋
- **Done Status**: Mark tasks as `Done` or `Not Done`. ✅❌
- **Timestamps**: Automatically adds a creation date to each task using `chrono`. 🕒
- **Persistent Storage**: Saves tasks as JSON in a file, appending new tasks without deleting old ones. 📂
- **Simple & Fast**: Built with Rust for performance and reliability. ⚡

---

## 🎯 How It Works

1. Run the CLI in your terminal. 🖥️
2. It prompts: `Enter your todo:`
   - Type your task (e.g., "Buy groceries 🛒").
3. It asks: `Is it Done? (yes/no)`
   - Respond with `yes` or `no`.
4. The task is saved with a timestamp in a JSON file (e.g., `todos.json`). 📅
5. New tasks are **appended** to the file, keeping all your todos safe! 😊

---

## 🛠️ Installation

Get started with Todo CLI in just a few steps! 🚧

### Prerequisites
- **Rust**: Make sure you have Rust installed. If not, install it from [rust-lang.org](https://www.rust-lang.org/). 🦀
- A terminal to run the CLI. 🖥️

### Steps
1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/todo_cli.git
