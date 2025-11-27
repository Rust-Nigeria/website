# Rust Nigeria website

Ensure you have Rust installed, if you don't, follow [these steps](https://rust-lang.org/tools/install/)

## Setup

### using just

Install `cargo-leptos` by running: `cargo install cargo-leptos`

Install `just` by running: `cargo install just`

`git clone git@github.com:Rust-Nigeria/website.git`

`cd WEBSITE`

RUN `just setup` to build the site

Run `just start` to start/run the site localy

---

### using docker

Install docker [checkout](https://docs.docker.com/engine/install/)

`git clone git@github.com:Rust-Nigeria/website.git`

`cd WEBSITE`

RUN `docker build -t rust-nigeria-website .` to build

RUN `docker run -p 3000:8080 rust-nigeria-website` to start/run the site localy

---

By default, you can access your local project at `http://localhost:3000`

Styling is done in [tailwindcss](https://tailwindcss.com/docs/installation) and [sass](https://sass-lang.com/documentation)

vscode configurations are added in to allow tailwind classes auto-completion

[Figma Design](https://www.figma.com/design/u1E8D25nhSb2nBBQChIV5u/Rust-Nigeria?m=auto&t=EJPY6TRsjBVMJkMk-6)

## Animation reference

[Animation reference](https://github.com/user-attachments/assets/44405bc9-3aee-49c9-8b90-66c6d20a54e0)

---

## Contributing Articles, Events, and Projects

If you want to contribute content to the Rust Nigeria website, please follow these guidelines for **articles, events, and projects**.

---

### Articles

When submitting an article, use the following JSON format:

```json
{
  "banner": "/assets/images/article-banners/default.png",
  "name": "Rust: Rewriting Experiences",
  "description": "Before I begin, let me get one thing out of the way. Rust isn't a programming language, it is an experience... based on Eze's journey with Rust.",
  "article_link": "https://dev.to/rustnigeria/rust-rewriting-experiences-2e74",
  "date": "2022-01-03T00:00:00.000Z",
  "authors": [
    {
      "name": "Bolu",
      "image": "/assets/images/members/bolu.png"
    }
  ],
  "tags": ["developer_story"]
}
```

**Notes:**

- `banner` is the path to your article image `assets/images/article-banners`.
- `name` is the article title.
- `description` is a short summary of the article.
- `article_link` is the external URL of the article.
- `date` should be in ISO 8601 format.
- `authors` is an array of contributors with their names and images `assets/images/members`.
- `tags` is an array of tags that best describe your article. Check for a tag that best fits your article or add a new one [here.](/src/types/articles.rs#L10)

---

### Events

When submitting an event, use the following JSON format:

```json
{
  "banner": "/assets/images/event-banners/rust-training-akwa-ibom.jpeg",
  "name": "1-week intensive Rust Developers Training",
  "description": "'The Rust developer’s on campus' Bootcamp kicks off on Monday at AKS University from the 21st of July - 21st of August 2025",
  "event_link": "https://x.com/promise_reckon/status/1946624594242060333",
  "date": "2025-07-21T16:00:00.000Z",
  "speakers": [
    {
      "name": "promise paul",
      "image": "assets/images/speakers/bolu.png",
      "portfolio": "url link"
    }
  ],
  "tags": ["workshop"]
}
```

**Notes:**

- `banner` is the path to the event image `assets/images/event-banners`.
- `name` is the event title.
- `description` is a short summary of the event.
- `event_link` is a URL to the event page or announcement.
- `date` should be in ISO 8601 format 2025-07-21T16:00:00.000Z.
- `speakers` is an array of speaker objects (if any) name, speaker image `assets/images/speakers` portfolio can be empty if non.
- `tags` is an array of tags that best describe your event. Check for a tag that best fits your event or add a new one [here.](/src/types/events.rs#L10)

---

### Projects

When submitting a project, use the following JSON format:

```json
[
  {
    "repo_url": "https://github.com/zerocore-ai/microsandbox",
    "banner": "https://opengraph.githubassets.com/1/zerocore-ai/microsandbox",
    "tags": ["ai"]
  }
]
```

**Notes:**

- `repo_url` is the GitHub repository link.
- `banner` is an image representing the project (e.g., Open Graph image or any image url).
- `tags` is an array of tags that best describe your project. Check for a tag that best fits your project or add a new one [here.](/src/types/projects.rs#L8)

---

### How to Submit

1. Fork the repository and create a branch for your contribution.
2. Add your content JSON in the correct file (e.g., `data/articles.json`, `data/events.json`, `data/projects.json`).
3. Commit your changes with a descriptive message.
4. Push your branch and open a pull request against `dev`.

Your contribution will be reviewed and merged once it follows the format and standards.

---

For other forms of contribution checkout [CONTRIBUTING.md](CONTRIBUTING.md)

This ensures all contributions are **structured consistently** and can be displayed properly on the website.

[live url](coming soon)

[STAGE url](https://nigeria-website.fly.dev/)
