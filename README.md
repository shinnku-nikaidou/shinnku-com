# Shinnku-com

A modern galgame resource website built with Next.js and HeroUI.

## Project Overview

Shinnku-com (真红小站) is a comprehensive galgame resource website that provides
access to various visual novel resources, including translated and original
Japanese games. The site features a clean, responsive interface with dark mode
support and efficient content organization.

[This site](https://www.shinnku.com) is one of the most popular galgame resource
websites in China, with over 1 million monthly active users. And the subtopic
website [https://galgame.dev](https://galgame.dev) also has over 1 million
monthly active users.

## Features

- **Responsive Design**: Optimized for desktop, tablet, and mobile experiences
- **Dark/Light Mode**: Support for user theme preferences
- **Content Management**: Organized catalog of galgame resources
- **Search Functionality**: Find specific games or categories quickly
- **Fast Loading**: Optimized performance using Next.js
- **Internationalization**: Built-in routing for `zh-cn`, `zh-tw`, and `en-us`

## Technologies Used

- [Next.js 14](https://nextjs.org/docs/getting-started) - React framework with
  app directory structure
- [HeroUI v2](https://heroui.com/) - UI components for React
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework
- [Tailwind Variants](https://tailwind-variants.org) - Variant management for
  Tailwind CSS
- [TypeScript](https://www.typescriptlang.org/) - Typed superset of JavaScript
- [Framer Motion](https://www.framer.com/motion/) - Animation library for React
- [next-themes](https://github.com/pacocoursey/next-themes) - Theme management
  for Next.js

## Getting Started

Clone the repository

```bash
git clone https://github.com/shinnku-nikaidou/shinnku-com.git
cd shinnku-com
```

Install dependencies Choose your preferred package manager

```bash
# Using npm
npm install

# Using yarn
yarn

# Using pnpm
pnpm install
```

Setup pnpm (if using) If you're using `pnpm`, add the following to your `.npmrc`
file:

```bash
public-hoist-pattern[]=*@heroui/*
```

Run development server

```bash
pnpm run dev
```

Open your browser and navigate to `http://localhost:3000` to see the app in
action.

### Backend Setup

Create a Python virtual environment and install the backend dependencies:

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# Build the Rust backend using the Python from this virtual environment
export PYO3_PYTHON=$(pwd)/.venv/bin/python3
cargo run -p shinnku-com-backend

# Do not set PYTHONHOME manually; the backend will read `pyvenv.cfg`
# to find the standard library when embedding Python.
```

## Internationalization

Next.js i18n routing generates localized paths for `zh-cn`, `zh-tw`, and `en-us`. The default locale is Simplified Chinese.

## License

Licensed under the
[MIT license](https://github.com/shinnku-nikaidou/shinnku-com/blob/main/LICENSE).
