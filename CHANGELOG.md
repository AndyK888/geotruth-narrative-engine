# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure and documentation
- Architecture documentation with Truth Engine design
- Backend services specification (FastAPI, PostGIS, Valhalla, Redis)
- Desktop application specification (Tauri v2, React, Rust)
- API reference documentation
- User guide with workflow explanations
- Development guide with setup instructions
- Security guidelines and best practices
- Contributing guidelines

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

---

## [0.1.5] - 2026-01-21

### Changed - UI/UX Modernization ✨

**Design System**
- Added Google Fonts (Poppins for headings, Open Sans for body text)
- Updated color palette to media/video production standards
- Enhanced glassmorphism effects with better contrast
- Improved focus states for keyboard navigation (WCAG AA compliance)
- Added reduced motion support for accessibility

**Icons**
- Replaced all emoji icons with professional Lucide SVG icons
- Added smooth icon animations on hover
- Improved visual consistency across the application

**Components**
- **App.tsx**: Enhanced main dashboard with SVG icons, improved status indicators
- **MapPacksModal**: Major refactoring - replaced emojis with SVG icons, consolidated CSS
- **ProjectList**: Added cursor-pointer to project cards for better UX
- **All Modals**: Improved backdrop blur and visual hierarchy

**Styling**
- Consolidated all modal styles into main design system (`index.css`)
- Removed separate `MapPacksModal.css` file
- Enhanced button hover states without layout shift
- Improved color contrast ratios for better readability

### Removed
- `desktop/src/components/MapPacksModal.css` (consolidated into main CSS)

### Technical
- Version bump: 0.1.4 → 0.1.5
- Code formatted with Prettier
- Build tested and verified

---

## [0.1.4] - 2026-01-21

### Added
- Project initialization
- Core documentation framework
- Basic project structure

---

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 0.1.0 | TBD | Initial release |

---

## Roadmap

### v0.2.0 - Core Pipeline
- [ ] Video import and analysis
- [ ] GPS parsing (GPX, NMEA)
- [ ] Time synchronization engine
- [ ] Basic map matching

### v0.3.0 - Intelligence Layer
- [ ] PostGIS integration
- [ ] POI discovery
- [ ] Field-of-view filtering
- [ ] Truth Bundle generation

### v0.4.0 - AI Narration
- [ ] Gemini integration
- [ ] Constraint-based prompting
- [ ] Chapter generation
- [ ] Script generation

### v0.5.0 - Export & Polish
- [ ] YouTube chapters export
- [ ] SRT subtitle export
- [ ] Interactive map export
- [ ] UI refinements

### v1.0.0 - Production Release
- [ ] Full feature set
- [ ] Performance optimization
- [ ] Cross-platform builds
- [ ] Documentation complete
