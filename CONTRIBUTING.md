# Contributing to Salvo

Thank you for your interest in contributing to Salvo! This document provides guidelines and instructions for contributing.

## Getting Started

1. **Fork the repository**
   ```bash
   git clone https://github.com/yourusername/salvo.git
   cd salvo
   ```

2. **Set up development environment**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Copy environment template
   cp .env.example .env

   # Build the project
   cargo build

   # Run tests
   cargo test
   ```

3. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Code Style

- **Format your code** before committing:
  ```bash
  cargo fmt
  ```

- **Run clippy** to catch common mistakes:
  ```bash
  cargo clippy
  ```

- **Follow Rust naming conventions**:
  - `snake_case` for functions, variables, modules
  - `PascalCase` for types, traits, enums
  - `SCREAMING_SNAKE_CASE` for constants

### Testing

- Write tests for new features
- Ensure all tests pass before submitting PR:
  ```bash
  cargo test
  ```

- Add integration tests in `tests/` directory
- Add unit tests in the same file as the code using `#[cfg(test)]`

### Documentation

- Add doc comments to public APIs:
  ```rust
  /// Calculate materials from reprocessed salvage
  ///
  /// # Arguments
  /// * `pool` - Database connection pool
  /// * `salvage_items` - List of salvage items to reprocess
  /// * `efficiency` - Reprocessing efficiency (0.5 to 0.69575)
  pub async fn calculate_materials(...)
  ```

- Update README.md if adding new features
- Update CHANGELOG.md with your changes

## Commit Guidelines

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```bash
feat(api): add endpoint for character skill integration

fix(db): resolve SQLite connection pool timeout issue

docs(readme): update Docker deployment instructions
```

## Pull Request Process

1. **Update documentation** for any changed functionality
2. **Add tests** for new features
3. **Ensure CI passes** (all tests, clippy, formatting)
4. **Update CHANGELOG.md** with your changes
5. **Request review** from maintainers

### PR Title Format
```
<type>: <description>
```

Example: `feat: Add EVE SSO authentication support`

### PR Description Template
```markdown
## Description
Brief description of what this PR does.

## Changes
- Change 1
- Change 2

## Testing
How was this tested?

## Related Issues
Fixes #123
```

## Areas for Contribution

### High Priority
- Complete SDE blueprint import parser
- Frontend UI implementation
- EVE SSO character authentication
- Multi-faction support

### Medium Priority
- Additional unit tests
- Performance optimizations
- Documentation improvements
- Example scripts and tutorials

### Good First Issues
Look for issues labeled `good first issue` in the issue tracker.

## Code Review Process

All submissions require review before being merged:

1. **Automated checks** must pass (CI, tests, clippy)
2. **Code review** by at least one maintainer
3. **Changes requested** should be addressed
4. **Final approval** before merge

## Development Guidelines

### Error Handling
- Use `ApiError` enum for API errors
- Use `anyhow::Result` for general errors
- Always provide context with `.context()` or `.map_err()`

### Database Queries
- Use SQLx query macros for compile-time verification
- Keep queries in `src/db/queries.rs`
- Use transactions for multi-step operations

### API Design
- Follow RESTful conventions
- Use proper HTTP status codes
- Return consistent JSON structures
- Include proper error messages

### Performance
- Avoid blocking operations in async functions
- Use connection pooling for database
- Cache expensive computations
- Profile before optimizing

## Building for Production

```bash
# Build optimized release
cargo build --release

# Run with release build
cargo run --release

# Build Docker image
docker build -t salvo-backend:latest .
```

## Questions?

- Open an issue for questions
- Check existing issues and PRs
- Review documentation in `/docs`

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Salvo! 🚀
