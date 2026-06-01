# Spec: provider

## Purpose

约束 ccstart 对 cc-switch provider 数据库的只读访问和查询结果。

## Requirements

### Requirement: Read-only database access
The system SHALL open the cc-switch SQLite database in read-only mode before querying provider data.

#### Scenario: Database path is resolved
- **GIVEN** a user invokes a command that needs provider data
- **WHEN** the database layer opens the cc-switch database
- **THEN** it uses the home-directory cc-switch database path with read-only SQLite flags

### Requirement: Claude provider queries
The system SHALL return only providers whose app_type is claude and preserve configured ordering.

#### Scenario: Provider names are listed
- **GIVEN** the providers table contains multiple app types
- **WHEN** ccstart requests provider names
- **THEN** only Claude provider names are returned in sort_index and name order
