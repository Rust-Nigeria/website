# Rust Nigeria website

## Setup

Install `cargo-leptos` by running:

`cargo install cargo-leptos --version 0.2.37 --locked`

Install `just` by running:

`cargo install just`

Then run

`just setup`

## Running your project

Run

`just start`

By default, you can access your local project at `http://localhost:3000`

Styling is done in [tailwindcss](https://tailwindcss.com/docs/installation) and [sass](https://sass-lang.com/documentation)

vscode configurations are added in to allow tailwind classes auto-completion

[Figma Design](https://www.figma.com/design/u1E8D25nhSb2nBBQChIV5u/Rust-Nigeria?m=auto&t=EJPY6TRsjBVMJkMk-6)

## Animation reference

[Animation reference](https://github.com/user-attachments/assets/44405bc9-3aee-49c9-8b90-66c6d20a54e0)

## TODO:

- Basic Blog structure
- Store posts somewhere <- Markdown or DB <- sqlite database + sqlx or seaORM or diesel + migrations with dbmate or markdown files
- Define Post struct
- Convert markdown to html(femark crate) | with Leptos server functions(add/update/remove post)
- Display the HTML with a Resource

# Deploy to Fly with GH Action
