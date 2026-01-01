# Contributing to GeoTruth

Thank you for your interest in contributing to GeoTruth! This document provides guidelines and information for contributors.

---

## 🤝 How to Contribute

### Types of Contributions

| Type | Description |
|------|-------------|
| 🐛 **Bug Reports** | Found a problem? Open an issue |
| ✨ **Feature Requests** | Have an idea? Start a discussion |
| 📝 **Documentation** | Fix typos, improve explanations |
| 🔧 **Code** | Bug fixes, new features |
| 🧪 **Testing** | Add tests, improve coverage |
| 🌍 **Translations** | Help localize the app |

---

## 🚀 Getting Started

### 1. Fork and Clone

```bash
# Fork via GitHub UI, then:
git clone https://github.com/YOUR_USERNAME/geotruth-narrative-engine.git
cd geotruth-narrative-engine
git remote add upstream https://github.com/geotruth/geotruth-narrative-engine.git
```

### 2. Set Up Development Environment

Follow the [Development Guide](docs/development/README.md) for complete setup instructions.

```bash
# Quick start
./scripts/setup-dev.sh
```

### 3. Create a Branch

```bash
git checkout develop
git pull upstream develop
git checkout -b feature/your-feature-name
```

**Branch naming:**
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation
- `refactor/` - Code refactoring
- `test/` - Test additions

---

## 📋 Pull Request Process

### Before Submitting

- [ ] Code follows project style guidelines
- [ ] All tests pass locally
- [ ] New code has test coverage
- [ ] Documentation is updated
- [ ] Commit messages follow conventions
- [ ] Branch is up-to-date with `develop`

### Submitting a PR

1. **Push your branch**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open a Pull Request** against `develop`

3. **Fill out the PR template**:
   - Description of changes
   - Screenshots (for UI changes)
   - Related issues
   - Testing done

4. **Wait for review**
   - Address feedback
   - Keep PR updated with `develop`

### PR Review Criteria

- ✅ Passes CI checks
- ✅ Has test coverage
- ✅ Follows code style
- ✅ Documentation updated
- ✅ No security issues
- ✅ Approved by at least one maintainer

---

## 💻 Code Guidelines

### TypeScript/React

```typescript
// ✅ Good
interface Props {
  eventId: string;
  onSelect: (id: string) => void;
}

export function EventCard({ eventId, onSelect }: Props) {
  const handleClick = useCallback(() => {
    onSelect(eventId);
  }, [eventId, onSelect]);

  return <div onClick={handleClick}>...</div>;
}

// ❌ Avoid
export function EventCard(props: any) {
  return <div onClick={() => props.onSelect(props.eventId)}>...</div>;
}
```

### Rust

```rust
// ✅ Good: Use Result, document public functions
/// Processes a GPS track file and returns parsed points.
///
/// # Errors
/// Returns `ParseError` if the file format is invalid.
pub fn parse_gpx(path: &Path) -> Result<Track, ParseError> {
    // implementation
}

// ❌ Avoid: panic, unwrap in library code
pub fn parse_gpx(path: &Path) -> Track {
    let content = fs::read_to_string(path).unwrap(); // Don't do this
}
```

### Python

```python
# ✅ Good: Type hints, docstrings, async
async def enrich_point(lat: float, lon: float) -> EnrichResponse:
    """
    Enrich a GPS point with geospatial context.
    
    Args:
        lat: Latitude in decimal degrees
        lon: Longitude in decimal degrees
        
    Returns:
        EnrichResponse with location context and POIs
    """
    ...

# ❌ Avoid: No types, no docs
def enrich_point(lat, lon):
    ...
```

---

## 📝 Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Formatting, no code change |
| `refactor` | Code change that's not a fix or feature |
| `test` | Adding or updating tests |
| `chore` | Maintenance, deps, CI |

### Examples

```bash
# Feature
feat(timeline): add zoom controls for event editing

# Bug fix
fix(gps): correct time offset calculation for negative values

# Documentation
docs(api): add examples for batch enrichment endpoint

# With body
feat(export): support SRT subtitle format

Add support for exporting narration as .srt subtitle files
with location markers embedded in each segment.

Closes #123
```

---

## 🐛 Bug Reports

### Before Reporting

1. **Search existing issues** - it may already be reported
2. **Update to latest** - the bug may be fixed
3. **Reproduce** - make sure it's consistent

### Report Template

```markdown
## Bug Description
Clear description of the bug.

## Steps to Reproduce
1. Go to '...'
2. Click on '...'
3. See error

## Expected Behavior
What should happen.

## Actual Behavior
What actually happens.

## Environment
- OS: [e.g., macOS 14.0]
- App Version: [e.g., 1.2.3]
- Video Format: [e.g., MP4/H.264]
- GPS Format: [e.g., GPX]

## Screenshots/Logs
If applicable.
```

---

## ✨ Feature Requests

Start a **Discussion** (not an Issue) for feature ideas.

### Include

1. **Use case**: What problem does this solve?
2. **Proposed solution**: How should it work?
3. **Alternatives**: What other options exist?
4. **Priority**: How important is this to you?

---

## 📜 Code of Conduct

### Our Pledge

We are committed to making participation in this project a harassment-free experience for everyone.

### Expected Behavior

- Be respectful and inclusive
- Accept constructive criticism
- Focus on what's best for the community
- Show empathy towards others

### Unacceptable Behavior

- Harassment, discrimination, or threats
- Trolling or insulting comments
- Personal or political attacks
- Publishing others' private information

### Enforcement

Violations may be reported to conductors@geotruth.io. All complaints will be reviewed and investigated.

---

## 🏆 Recognition

Contributors are recognized in:

- `CONTRIBUTORS.md` file
- Release notes
- Annual contributor spotlight

---

## ❓ Questions?

- **Discord**: [#contributors channel](https://discord.gg/geotruth)
- **Discussions**: GitHub Discussions
- **Email**: contributors@geotruth.io

---

Thank you for contributing! 🎉
