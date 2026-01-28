# Money Bill

A modern, cross-platform personal finance management application built with **Tauri v2** and **Vue 3**.

## Features

- 📊 **Dashboard**: Overview of your financial status.
- 💰 **Asset Management**: Track various assets and accounts.
- 💸 **Bill Tracking**: Easily record income and expenses.
- 📉 **Budgeting**: Set and monitor monthly budgets.
- 📅 **History**: View detailed transaction history.
- 🔒 **Data Privacy**: Local SQLite database ensures your financial data stays on your device.

## Tech Stack

- **Core**: [Rust](https://www.rust-lang.org/) & [Tauri](https://tauri.app/) (v2)
- **Frontend**: [Vue 3](https://vuejs.org/) & [TypeScript](https://www.typescriptlang.org/)
- **UI Framework**: [Vuetify 3](https://vuetifyjs.com/) & [Tailwind CSS](https://tailwindcss.com/)
- **Visualization**: [ECharts](https://echarts.apache.org/)
- **Database**: [SQLite](https://www.sqlite.org/)

## Prerequisites

Ensure you have the following installed:

- [Node.js](https://nodejs.org/) (v16+)
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install)

## Getting Started

1. **Clone the repository**
   ```bash
   git clone <repository_url>
   cd money-bill
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Run in development mode**
   ```bash
   pnpm tauri dev
   ```

   This will start the Vite dev server and open the Tauri application window.

## Building for Production

To build the application for your operating system:

```bash
pnpm tauri build
```

The build artifacts will be located in `src-tauri/target/release/bundle/`.

## Project Structure

- `src/`: Frontend Vue application source code
  - `components/`: UI components (AddBill, Assets, Budget, etc.)
  - `store/`: State management
  - `router.ts`: Application routing
- `src-tauri/`: Backend Rust application and Tauri configuration
