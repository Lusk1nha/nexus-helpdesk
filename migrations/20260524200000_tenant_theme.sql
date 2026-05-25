-- Adds a theme preference to each tenant.
-- The value must match one of the ThemeId values defined in packages/theme/src/themes.ts.
-- Defaults to "midnight" (the application's default theme).

ALTER TABLE tenants
    ADD COLUMN theme VARCHAR(50) NOT NULL DEFAULT 'midnight';
