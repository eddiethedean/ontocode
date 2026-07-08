# MENUS.md

# Protégé Menu System Reverse Engineering

## Purpose

The Protégé menu system provides global access to ontology project actions, editing commands, reasoner controls, refactoring tools, plugin commands, window management, and help resources. Unlike tab-specific views, menu actions usually operate at the application, project, ontology, or selected-entity level.

This document describes the functional role of Protégé menus as a reverse-engineering reference for building OntoCode feature parity and modernization.

---

# High-Level Menu Structure

Protégé Desktop commonly organizes global actions into menus similar to:

```text
File
Edit
Active Ontology
Refactor
Reasoner
Tools
Window
Help
```

Plugin installations may add new top-level menus or add commands under existing menus.

---

# Menu Design Principles

Protégé menus are designed around these assumptions:

1. Ontology editing is project-centered.
2. Most edits apply to the currently active ontology or selected entity.
3. Reasoning is a global workspace operation.
4. Refactoring should be explicit and discoverable.
5. Plugins should be able to contribute commands.
6. Advanced users need access to low-frequency but important actions.

---

# File Menu

## Purpose

The File menu manages ontology projects, ontology files, imports, saving, loading, exporting, and application-level file operations.

## Common Responsibilities

- Create new ontology project
- Open ontology project
- Open recent project
- Save ontology
- Save ontology as
- Save all
- Close project
- Import ontology
- Export ontology
- Reload ontology
- Manage physical ontology documents
- Exit application

## Typical Actions

### New Project / New Ontology

Creates a new Protégé project, usually backed by a new ontology document.

Expected user decisions:

- Ontology IRI
- Version IRI
- Physical file location
- Serialization format
- Prefix configuration

### Open

Loads an existing ontology file or project.

Supported ontology formats may include:

- RDF/XML
- OWL/XML
- Turtle
- Manchester Syntax
- Functional Syntax
- OBO, depending on plugins and import support

### Open Recent

Provides quick access to recently opened ontology projects.

### Save

Writes current ontology changes to the known physical location.

### Save As

Allows the user to choose a new location or serialization format.

### Save All

Saves all dirty ontologies in the current project, including imports when editable.

### Close

Closes the current ontology project.

### Import

Adds an ontology import declaration or loads an external ontology into the project.

### Export

Serializes ontology content to another file or format.

### Exit / Quit

Closes the Protégé application.

## OntoCode Parity Requirements

- [ ] New ontology command
- [ ] Open ontology command
- [ ] Recent projects list
- [ ] Save
- [ ] Save as
- [ ] Save all
- [ ] Close project
- [ ] Import ontology
- [ ] Export ontology
- [ ] Dirty-state tracking
- [ ] Unsaved-changes warning
- [ ] Serialization format selection
- [ ] File recovery support

## OntoCode Modernization Opportunities

- Git-aware save state
- Workspace-level project files
- Autosave with semantic checkpoints
- Save conflict resolution
- Import source trust indicators
- Format conversion preview
- Cloud and local workspace parity

---

# Edit Menu

## Purpose

The Edit menu exposes generic editing commands and selection-based actions.

## Common Responsibilities

- Undo
- Redo
- Cut
- Copy
- Paste
- Delete
- Select all
- Find
- Replace
- Entity search
- Preferences

## Typical Actions

### Undo

Reverts the most recent ontology edit.

Ontology edits may include:

- Adding a class
- Deleting a property
- Changing an annotation
- Adding an axiom
- Removing an axiom
- Renaming an entity

### Redo

Reapplies an undone ontology edit.

### Cut / Copy / Paste

Used for text fields, entity references, axioms, and sometimes tree selections.

### Delete

Deletes selected entities, axioms, annotations, or text depending on context.

### Find

Searches for entities, labels, IRIs, annotations, or text.

### Preferences

Opens application preferences.

Preference categories may include:

- Rendering
- Reasoner behavior
- Editor behavior
- Plugin settings
- UI layout
- Entity display
- New entity creation policy

## OntoCode Parity Requirements

- [ ] Undo
- [ ] Redo
- [ ] Clipboard support
- [ ] Delete selected object
- [ ] Global search
- [ ] Preferences dialog
- [ ] Context-sensitive enablement
- [ ] Text-field editing support
- [ ] Entity-aware copy/paste

## OntoCode Modernization Opportunities

- Command palette integration
- Multi-step undo timeline
- Semantic undo labels
- Keyboard-first editing
- Search across project, imports, Git history, and documentation
- AI-assisted find/replace for ontology patterns

---

# Active Ontology Menu

## Purpose

The Active Ontology menu manages actions that apply specifically to the ontology currently selected as the active editable ontology.

## Common Responsibilities

- Ontology annotations
- Ontology IRI management
- Prefix management
- Import management
- Ontology metrics
- Ontology format settings
- Ontology closure view
- Physical document mapping

## Typical Actions

### Edit Ontology Annotations

Allows the user to add, edit, or remove annotations on the ontology itself.

Examples:

- rdfs:label
- rdfs:comment
- dc:title
- dc:creator
- owl:versionInfo
- license metadata

### Manage Imports

Adds, removes, or reloads imported ontologies.

Import behavior must account for:

- Logical IRI
- Physical IRI
- Missing imports
- Circular imports
- Import closure
- Editable versus read-only imports

### Manage Prefixes

Controls prefix mappings used for compact entity rendering.

Examples:

```text
owl:  http://www.w3.org/2002/07/owl#
rdf:  http://www.w3.org/1999/02/22-rdf-syntax-ns#
rdfs: http://www.w3.org/2000/01/rdf-schema#
xsd:  http://www.w3.org/2001/XMLSchema#
```

### Ontology Metrics

Displays metrics such as:

- Number of classes
- Number of object properties
- Number of data properties
- Number of annotation properties
- Number of individuals
- Number of axioms
- Logical axiom count
- Annotation axiom count
- Imported ontology count

### Set Active Ontology

In multi-ontology projects, determines where new axioms and entities are added.

## OntoCode Parity Requirements

- [ ] Active ontology selector
- [ ] Ontology annotation editor
- [ ] Prefix editor
- [ ] Import manager
- [ ] Ontology metrics
- [ ] Logical/physical IRI mapping
- [ ] Editable import support
- [ ] Missing import diagnostics
- [ ] Reload imports

## OntoCode Modernization Opportunities

- Visual import graph
- Import health dashboard
- Dependency lockfile
- Package-manager-style ontology imports
- Prefix conflict detection
- Metadata quality scoring
- AI-suggested ontology annotations

---

# Refactor Menu

## Purpose

The Refactor menu provides higher-level ontology transformation tools that preserve or intentionally alter semantics across multiple axioms.

## Common Responsibilities

- Rename entity
- Move entity
- Merge entities
- Delete entity safely
- Extract module
- Convert entity type
- Replace entity references
- Normalize labels
- Change entity IRIs

## Typical Actions

### Rename Entity

Changes an entity IRI and updates references.

Expected behavior:

- Update all axioms referencing the entity
- Preserve annotations when appropriate
- Warn about imported or read-only references
- Optionally update labels

### Move Entity

Changes an entity's position in a hierarchy.

Examples:

- Move class under a different superclass
- Move property under a different superproperty

### Merge Entities

Combines two or more ontology entities.

Expected behavior:

- Consolidate axioms
- Merge annotations
- Resolve duplicate labels
- Warn about semantic conflicts

### Delete Entity

Deletes an entity and related axioms.

A safe delete should show:

- Direct usages
- Indirect usages
- Referencing axioms
- Imported references
- Impact summary

### Extract Module

Creates a smaller ontology module from selected entities and dependencies.

Potential module strategies:

- Top module
- Bottom module
- Star module
- Signature-based extraction

## OntoCode Parity Requirements

- [ ] Rename entity
- [ ] Move entity
- [ ] Merge entities
- [ ] Safe delete
- [ ] Replace entity
- [ ] Extract module
- [ ] Preview refactor impact
- [ ] Undoable refactors
- [ ] Refactor conflict warnings

## OntoCode Modernization Opportunities

- IDE-quality rename previews
- Refactor diff viewer
- Semantic impact analysis
- Git branch refactor workflow
- AI-assisted ontology cleanup
- Bulk refactoring recipes
- Typed refactor APIs for plugins

---

# Reasoner Menu

## Purpose

The Reasoner menu controls ontology classification, consistency checking, reasoner selection, and inference-related operations.

## Common Responsibilities

- Select reasoner
- Start reasoner
- Stop reasoner
- Synchronize reasoner
- Classify ontology
- Check consistency
- Precompute inferences
- Show inferred hierarchy
- Configure reasoner
- Dispose reasoner

## Typical Actions

### Select Reasoner

Chooses the active reasoner implementation.

Common options may include:

- HermiT
- Pellet
- Fact++
- ELK
- Structural reasoner
- Plugin-provided reasoners

### Start Reasoner

Initializes the selected reasoner for the active ontology.

### Synchronize Reasoner

Pushes ontology changes into the reasoner and recomputes inferences.

### Classify

Computes inferred class and property hierarchies.

### Check Consistency

Determines whether the ontology is logically consistent.

### Explain Inconsistency

Opens explanation tooling to identify axioms contributing to inconsistency or unsatisfiable classes.

### Configure Reasoner

Opens reasoner-specific settings.

Settings may include:

- Timeout
- Incremental reasoning
- Fresh entity policy
- Explanation limits
- Precomputation options
- Logging verbosity

## OntoCode Parity Requirements

- [ ] Reasoner selection
- [ ] Reasoner lifecycle management
- [ ] Synchronize reasoner
- [ ] Classify ontology
- [ ] Consistency checking
- [ ] Unsatisfiable class detection
- [ ] Inferred hierarchy views
- [ ] Explanation integration
- [ ] Reasoner configuration
- [ ] Long-running task cancellation

## OntoCode Modernization Opportunities

- Rust-native reasoner abstraction
- Reasoner progress UI
- Incremental live reasoning
- Background classification
- Explanation graph visualization
- Reasoning performance profiler
- Reasoner comparison mode
- AI explanation summaries

---

# Tools Menu

## Purpose

The Tools menu collects advanced utilities, plugin actions, validation tools, visualization tools, scripting tools, and project-specific commands.

## Common Responsibilities

- Plugin-provided commands
- Ontology validation
- Ontology metrics
- Visualization tools
- Entity reports
- Batch operations
- Import/export helpers
- Scripting or automation

## Typical Actions

### Ontology Metrics

Displays detailed ontology statistics.

### Visualization

Launches graph-style views when plugins are installed.

Examples:

- OntoGraf
- OWLViz
- VOWL-style views

### Validation

Checks ontology for modeling issues beyond formal logical consistency.

Possible validation categories:

- Missing labels
- Duplicate labels
- Orphan classes
- Cycles where unexpected
- Missing domains/ranges
- Unused properties
- Deprecated entity usage

### Plugin Actions

Plugins may contribute arbitrary tool commands.

## OntoCode Parity Requirements

- [ ] Tool command registry
- [ ] Plugin-contributed tools
- [ ] Metrics command
- [ ] Validation command
- [ ] Visualization launch commands
- [ ] Batch operation support

## OntoCode Modernization Opportunities

- Scriptable command runner
- Task pipeline automation
- Integrated linting
- Ontology quality dashboards
- Plugin marketplace
- Workspace tasks similar to VS Code tasks
- AI-generated repair suggestions

---

# Window Menu

## Purpose

The Window menu manages workspace layout, tabs, panels, docking state, and view visibility.

## Common Responsibilities

- Open tabs
- Close tabs
- Reset layout
- Show/hide views
- Manage perspectives
- Restore default workspace
- Switch active window

## Typical Actions

### Reset Layout

Restores the default Protégé workspace layout.

### Show View

Displays a hidden dockable view.

### Switch Tab

Moves focus to a major workspace tab.

### Manage Perspectives

Some configurations may allow saved workspace layouts or perspectives.

## OntoCode Parity Requirements

- [ ] Show/hide panels
- [ ] Reset layout
- [ ] Switch tabs
- [ ] Persist layout
- [ ] Restore default workspace
- [ ] Plugin-contributed views
- [ ] Floating and docked panel support

## OntoCode Modernization Opportunities

- Named workspaces
- Modeling/reasoning/review perspectives
- Keyboard-driven layout switching
- Split editors
- Multi-window support
- Workspace layout sync
- Per-project layout configuration

---

# Help Menu

## Purpose

The Help menu provides access to documentation, tutorials, diagnostics, updates, and application information.

## Common Responsibilities

- Open documentation
- Show getting started material
- Show about dialog
- Show version information
- Plugin information
- Error logs
- Update checks
- Community links

## Typical Actions

### Documentation

Opens official Protégé documentation.

### About

Displays:

- Application version
- Java version
- Build information
- Plugin versions
- License information

### Error Log

Shows application errors, stack traces, or diagnostic logs.

### Plugin Information

Displays installed plugins and versions.

## OntoCode Parity Requirements

- [ ] Documentation links
- [ ] About dialog
- [ ] Version information
- [ ] Plugin list
- [ ] Error log viewer
- [ ] Diagnostic export
- [ ] Support links

## OntoCode Modernization Opportunities

- Built-in learning mode
- Contextual help side panel
- AI help assistant
- Interactive ontology modeling tutorials
- One-click diagnostic bundle
- Release notes viewer
- Plugin health report

---

# Context Menus

## Purpose

Context menus expose actions relevant to the selected UI object.

They are essential because many Protégé workflows start from hierarchy trees, entity lists, or axiom rows rather than from the top-level menu bar.

## Common Context Menu Locations

- Class hierarchy tree
- Object property tree
- Data property tree
- Annotation property tree
- Individual list
- Axiom list
- Annotation rows
- Import rows
- Search results
- Entity usage views

## Common Context Actions

- Create child class
- Create sibling class
- Rename
- Delete
- Add annotation
- Copy IRI
- Copy short form
- Show usages
- Show in hierarchy
- Move entity
- Add superclass
- Add equivalent class
- Add disjoint class
- Add property assertion

## OntoCode Parity Requirements

- [ ] Entity-aware context menus
- [ ] Axiom-aware context menus
- [ ] Tree context menus
- [ ] Search result context menus
- [ ] Plugin-contributed context commands
- [ ] Keyboard-accessible context actions

## OntoCode Modernization Opportunities

- Command palette parity for every context action
- Inline quick actions
- AI-suggested context actions
- Recently used actions
- Batch context operations
- Safe-delete preview from context menu

---

# Menu Enablement Rules

## Purpose

Menu actions should be enabled, disabled, or hidden based on current state.

## Examples

Save should be enabled only when there are unsaved changes.

Rename should be enabled only when an entity is selected and editable.

Reasoner synchronization should be enabled only when a reasoner is selected and ontology changes exist.

Delete should be disabled for read-only imported entities unless the delete only affects local referencing axioms.

Export should be enabled when an ontology project is loaded.

## OntoCode Requirements

- [ ] Central command registry
- [ ] Declarative enablement rules
- [ ] Context-aware action state
- [ ] Read-only import protection
- [ ] Dirty-state awareness
- [ ] Reasoner-state awareness
- [ ] Selection-state awareness

---

# Keyboard Shortcuts

## Purpose

Keyboard shortcuts make menu actions available to power users.

## Common Shortcut Categories

- File operations
- Edit operations
- Search
- Navigation
- Reasoner synchronization
- Entity creation
- Entity deletion
- Workspace switching

## OntoCode Requirements

- [ ] Shortcut registry
- [ ] User-customizable keybindings
- [ ] Conflict detection
- [ ] Command palette integration
- [ ] Keyboard shortcut documentation
- [ ] Platform-aware defaults

## Modernization Opportunity

OntoCode should treat every menu action as a command with:

- stable command ID
- label
- description
- shortcut
- icon
- category
- enablement rule
- execution handler
- plugin contribution metadata

This would make OntoCode more like VS Code or JetBrains IDEs than a traditional desktop editor.

---

# Plugin Menu Contributions

## Purpose

Protégé plugins can extend the UI by contributing menu actions, views, tabs, reasoners, and tools.

## Plugin Contribution Types

- Top-level menu
- Submenu
- Tool action
- View action
- Context menu action
- Reasoner menu entry
- Help/about entry

## OntoCode Requirements

- [ ] Plugin command contribution API
- [ ] Plugin view contribution API
- [ ] Plugin menu placement rules
- [ ] Permission or trust model
- [ ] Plugin diagnostics
- [ ] Plugin enable/disable support
- [ ] Plugin version compatibility checks

## Modernization Opportunity

OntoCode should support plugin-defined commands through a manifest format.

Example:

```json
{
  "contributes": {
    "commands": [
      {
        "id": "ontocode.validateLabels",
        "title": "Validate Labels",
        "category": "Ontology Quality",
        "menus": ["tools", "entity/context"],
        "when": "workspace.hasOntology"
      }
    ]
  }
}
```

---

# Recommended OntoCode Menu Model

OntoCode should not directly clone Protégé's menus. It should preserve feature coverage while modernizing the interaction model.

## Proposed Top-Level Menus

```text
File
Edit
View
Navigate
Ontology
Reasoner
Refactor
Tools
Plugins
Window
Help
```

## Why Add View?

Protégé blends view control into Window. Modern IDEs usually distinguish:

- View: visible panels, zoom, display options
- Window: application windows and workspace layout

## Why Add Navigate?

Ontology projects need fast movement between:

- classes
- properties
- individuals
- axioms
- usages
- imports
- errors
- inferred entities

## Why Add Plugins?

A first-class plugin menu makes extension management visible and trusted.

---

# OntoCode Command Architecture

Every menu item should be backed by a command object.

## Command Metadata

Each command should define:

- ID
- Title
- Category
- Description
- Icon
- Shortcut
- Menu placement
- Context menu placement
- Enablement condition
- Required permissions
- Undo behavior
- Telemetry category
- Plugin owner, if applicable

## Command Execution

Commands should execute through a centralized command bus.

Benefits:

- menu integration
- toolbar integration
- context menu integration
- command palette integration
- keyboard shortcut integration
- macro support
- plugin support
- testability

---

# Feature Parity Checklist

## File

- [ ] New ontology
- [ ] Open ontology
- [ ] Open recent
- [ ] Save
- [ ] Save as
- [ ] Save all
- [ ] Close project
- [ ] Import ontology
- [ ] Export ontology
- [ ] Exit application

## Edit

- [ ] Undo
- [ ] Redo
- [ ] Cut
- [ ] Copy
- [ ] Paste
- [ ] Delete
- [ ] Find
- [ ] Preferences

## Active Ontology / Ontology

- [ ] Ontology annotations
- [ ] Prefix management
- [ ] Import management
- [ ] Ontology metrics
- [ ] Active ontology selection
- [ ] Physical IRI mapping

## Refactor

- [ ] Rename entity
- [ ] Move entity
- [ ] Merge entities
- [ ] Safe delete
- [ ] Replace entity
- [ ] Extract module

## Reasoner

- [ ] Select reasoner
- [ ] Start reasoner
- [ ] Stop reasoner
- [ ] Synchronize reasoner
- [ ] Classify
- [ ] Check consistency
- [ ] Explain inconsistency
- [ ] Configure reasoner

## Tools

- [ ] Metrics
- [ ] Validation
- [ ] Visualization
- [ ] Batch tools
- [ ] Plugin tools

## Window / View

- [ ] Show views
- [ ] Hide views
- [ ] Reset layout
- [ ] Switch tabs
- [ ] Save layout
- [ ] Restore layout

## Help

- [ ] Documentation
- [ ] About
- [ ] Plugin list
- [ ] Error log
- [ ] Diagnostics
- [ ] Release notes

---

# Implementation Guidance for OntoCode

## Minimum Viable Menu System

For an early OntoCode release, implement:

1. File
2. Edit
3. Ontology
4. Reasoner
5. Refactor
6. View
7. Help

Back every item with a command registry.

## Version 1.0 Menu System

For OntoCode 1.0, add:

- plugin-contributed commands
- command palette
- customizable shortcuts
- context menus
- workspace layout commands
- Git-aware save/export commands
- semantic refactor previews

## Beyond Protégé

Long-term OntoCode should support:

- AI-authored ontology edits as commands
- command macros
- batch ontology transformations
- shared command histories
- collaborative command review
- scripted command execution
- command-level permissions
- Git commit generation from command history

---

# Summary

Protégé's menu system is functional, mature, and deeply tied to the ontology editing workflow. Its menus expose the core capabilities needed for ontology engineering: project management, editing, active ontology configuration, refactoring, reasoning, tools, workspace layout, and help.

For OntoCode, the goal should not be a pixel-for-pixel clone. The goal should be command-level feature parity with a more modern architecture. Every Protégé menu action should map to an OntoCode command, and every command should be accessible through menus, context menus, keyboard shortcuts, the command palette, plugins, and automation.

This approach preserves Protégé's proven ontology engineering coverage while making OntoCode feel like a modern IDE for semantic systems.
