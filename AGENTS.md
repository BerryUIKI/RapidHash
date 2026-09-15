# RapidHash Contributor & Agent Guidelines (AGENTS.md)

This document establishes the operational rules, branch hierarchy, code quality standards, and development workflows for AI agents, automated systems, and human contributors working on RapidHash.

---

## 1. Branch Architecture & Protection Model

RapidHash uses a strict **two-tier protected branch model**:

```text
       [Feature / Topic Branches]
                   │
           (Pull Request)
                   ▼
         dev (Active Integration)  <─── Protected: All work lands here first
                   │
           (Milestone PR)
                   ▼
       main (Production / Stable)  <─── Protected: Production releases only
```

### 1.1 Protected Branches: `main` and `dev`

Both `main` and `dev` are protected branches with the following GitHub rules:
- **Direct pushes are strictly prohibited**: All modifications must arrive via a Pull Request.
- **Admin enforcement enabled**: Protections apply to repository administrators and single maintainers.
- **Linear history required**: Rebase or squash merges only; merge commits are disabled.
- **Force pushes and branch deletions disabled**.
- **Resolved conversations required** before merge.

### 1.2 Branch Roles & Promotion Policy

1. **`main` (Production / Stable)**:
   - Contains only verified, production-ready, releasable code.
   - **Never target `main` directly for feature or bugfix PRs**.
   - Changes reach `main` **only** through promotion Pull Requests from `dev` once milestones are completed and verified.

2. **`dev` (Active Integration & Staging)**:
   - The primary integration branch where day-to-day development occurs.
   - **All topic branches (`feat/*`, `fix/*`, `docs/*`, `refactor/*`, `test/*`) must branch off `dev` and open Pull Requests targeting `dev`**.
   - Must build cleanly and pass all automated tests at every merged PR.

3. **Topic Branches**:
   - Always create from the latest `dev` (`git checkout dev && git pull origin dev && git checkout -b <type>/<description>`).
   - Naming convention: `<type>/<short-description>` (e.g. `feat/hash-export-dialog`, `fix/crc-filename-spacing`, `docs/agent-instructions`).
   - Delete topic branches immediately after merging into `dev`.

---

## 2. Language & Commit Standards

- **Canonical Language**: All code identifiers, comments, documentation, commit messages, branch names, and Pull Request titles/descriptions targeting `dev` and `main` must be **entirely in English**.
- **Conventional Commits**: Every commit and PR title must strictly adhere to the Conventional Commits specification:
  - `feat(...)`: New user-facing feature or enhancement.
  - `fix(...)`: Bug fix or error resolution.
  - `docs(...)`: Documentation additions or revisions.
  - `refactor(...)`: Code changes that neither fix a bug nor add a feature.
  - `perf(...)`: Performance improvements.
  - `test(...)`: Adding or updating automated tests.
  - `build(...)` / `ci(...)`: Build scripts, dependencies, or workflow changes.
  - `chore(...)`: Routine repository maintenance.
- **Non-Interactive Git Commits**:
  - Always commit using `--no-gpg-sign` (e.g., `git commit --no-gpg-sign -m "..."`) to prevent automated processes from blocking on interactive GPG pinentry prompts.

---

## 3. Pull Request & Merging Workflow

1. **Start from `dev`**:
   ```bash
   git checkout dev
   git pull origin dev
   git checkout -b <type>/<description>
   ```

2. **Develop and Validate Locally**:
   Ensure all verification gates pass (see Section 4).

3. **Commit with Conventional Commit Message**:
   ```bash
   git add -A
   git commit --no-gpg-sign -m "<type>(<scope>): <summary in English>"
   ```

4. **Push Topic Branch and Open PR targeting `dev`**:
   ```bash
   git push -u origin <type>/<description>
   gh pr create --base dev --title "<type>(<scope>): <summary>" --body "<description>"
   ```

5. **Merge PR into `dev`**:
   - Use squash merge with an informative Conventional Commit title:
     ```bash
     gh pr merge <pr-number> --squash
     ```
   - Update local `dev` and delete the topic branch:
     ```bash
     git checkout dev
     git pull origin dev
     git branch -d <type>/<description>
     ```

6. **Promote `dev` to `main` (Milestone / Release only)**:
   - Once all tasks of a milestone or release are complete and verified on `dev`, open a promotion PR:
     ```bash
     gh pr create --base main --head dev --title "release: vX.Y.Z" --body "..."
     gh pr merge <pr-number> --squash
     ```

---

## 4. Verification Gates (Mandatory Before Every PR)

Before creating or merging any Pull Request targeting `dev` or `main`, the following four verification gates **must pass with zero errors and zero warnings**:

### 4.1 Rust Core & Desktop Tests
```bash
cargo test --workspace
```
*Must pass all unit tests, integration vectors, path security checks, manifest parsers, and localization parity tests.*

### 4.2 Rust Clippy Static Analysis
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
*Must pass with 0 warnings.*

### 4.3 Rust Code Formatting
```bash
cargo fmt --all --check
```
*(If unformatted, run `cargo fmt --all` to automatically format).*

### 4.4 Desktop Frontend Build & TypeScript Typecheck
```bash
pnpm --filter @rapidhash/desktop build
# or run inside apps/desktop:
pnpm build
```
*Must compile cleanly with zero TypeScript errors and produce production Vite bundles.*

---

## 5. Localization (i18n) Rules

RapidHash uses Mozilla Fluent (`.ftl`) for canonical localization:
- **Catalogs**:
  - `locales/en/`: Canonical English source of truth (`common.ftl`, `cli.ftl`, `desktop.ftl`).
  - `locales/zh-CN/`: Simplified Chinese translation.
- **Structural Parity**:
  - Any new translation key added to `locales/en/` must simultaneously be added to `locales/zh-CN/` with matching message identifiers, attributes, and variable names.
  - The `catalog_parity.rs` test suite automatically verifies that `en` and `zh-CN` remain in 1:1 parity.
- **Frontend & Backend Sync**:
  - Add fallback strings to `apps/desktop/src/i18n.ts`.
  - Include keys in `get_locale_strings` within `apps/desktop/src-tauri/src/lib.rs`.

---

## 6. Clean-Room Implementation & Licensing

- **License**: Apache-2.0.
- **Clean-Room Boundary**: RapidCRC Unicode and OpenHashTab are historical and UI reference points. Contributors and AI agents must **never** copy, translate, or paste code from GPL-licensed software.
- All algorithms and file operations must be implemented cleanly based on public RFCs, standards, and observable behavior.
