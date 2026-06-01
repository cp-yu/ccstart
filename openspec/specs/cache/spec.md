# Spec: cache

## Purpose

约束 Claude settings 缓存的生成、更新、差异报告和清理行为。

## Requirements

### Requirement: Read-through cache freshness
The system SHALL ensure each selected provider has a current cache file before launching Claude.

#### Scenario: Cache is created or reused
- **GIVEN** a selected provider has settings_config data
- **WHEN** ccstart prepares the provider for Claude
- **THEN** it writes a cache file when missing and reuses it when the content hash is unchanged

### Requirement: Field-level update reporting
The system SHALL report top-level JSON fields that changed when an existing cache file is updated.

#### Scenario: Updated cache reports changed fields
- **GIVEN** an existing cache file differs from provider settings_config
- **WHEN** the cache manager refreshes the file
- **THEN** it returns the cache path and the changed top-level fields

### Requirement: Stale cache cleanup
The system SHALL remove cached configuration files whose decoded names are no longer present in the provider list.

#### Scenario: Update removes stale files
- **GIVEN** a cache file exists for a provider name absent from the database result
- **WHEN** ccstart update completes synchronization
- **THEN** the stale cache file is removed and reported
