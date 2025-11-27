# Contributing to Rust Nigeria Website

Thank you for your interest in contributing to the Rust Nigeria website! This document will guide you on how to set up the project, submit changes, and ensure a smooth contribution process.

---

## Table of Contents

1. [How to Contribute](#how-to-contribute)
2. [Project Setup](#project-setup)
3. [Coding Standards](#coding-standards)
4. [Submitting Changes](#submitting-changes)
5. [Reporting Issues](#reporting-issues)
6. [Additional Resources](#additional-resources)

---

## How to Contribute

There are several ways you can contribute:

- Reporting bugs or issues
- Suggesting new features
- Submitting pull requests with code or content improvements
- summiting project,event or blog post
- Improving documentation or design

Please follow the guidelines below to make your contribution process smooth.

---

## Project Setup

### Using Just

1. Install required tools:

   ```bash
   cargo install cargo-leptos
   cargo install just
   ```

2. Build the site:

   ```bash
   just setup
   ```

3. Start the site locally:

   ```bash
   just start
   ```

4. Open your browser at: [http://localhost:3000](http://localhost:3000)

---

### Using Docker

1. Install Docker: [Docker Installation Guide](https://docs.docker.com/engine/install/)
2. Clone the repository:

   ```bash
   git clone git@github.com:Rust-Nigeria/website.git
   cd website
   ```

3. Build the Docker image:

   ```bash
   docker build -t rust-nigeria-website .
   ```

4. Run the Docker container:

   ```bash
   docker run -p 3000:8080 rust-nigeria-website
   ```

5. Open your browser at: [http://localhost:3000](http://localhost:3000)

---

## Coding Standards

- **Styling:** TailwindCSS and Sass are used for styling.

  - Tailwind classes autocomplete is enabled in VSCode.

- **HTML/Leptos:** Follow existing component structure.
- **Commit messages:** Use clear, concise messages that describe your changes.
- Ensure you run
  `rustc cargo_tool.rs`
  and
  `./cargo_tool.exe` for windows or
  `./cargo_tool` linux and mac before sending a PR

---

## Submitting Changes

1. Fork the repository and create a feature branch:

   ```bash
   git checkout -b <my-feature-branch>
   ```

2. Make your changes locally and test them.
3. Commit your changes:

   ```bash
   git commit -m "Add meaningful description of change"
   ```

4. Push your branch and open a pull request against the `dev` branch.

---

## Reporting Issues

If you encounter any bugs or issues:

1. Check existing issues to avoid duplicates.
2. Open a new issue with:

   - Clear title
   - Steps to reproduce
   - Screenshots or links if applicable

---

## Additional Resources

- [Figma Design (Use as a guide)](https://www.figma.com/design/u1E8D25nhSb2nBBQChIV5u/Rust-Nigeria?m=auto&t=EJPY6TRsjBVMJkMk-6)
- [Animation Reference](https://github.com/user-attachments/assets/44405bc9-3aee-49c9-8b90-66c6d20a54e0)
  [https://www.rustnigeria.org](rustnigeria.org)
  [STAGEING url](https://nigeria-website.fly.dev/)
