# Release Guide

## Publishing a New Version

This Action uses Docker images from GitHub Container Registry. To publish a new version:

### 1. Update Version

Update version in these files:
- `Cargo.toml` - Change `version = "x.y.z"`
- `Dockerfile` - Update `LABEL version="x.y.z"`

### 2. Create and Push Tag

```bash
# Create a new tag
git tag -a v1.0.0 -m "Release v1.0.0"

# Push the tag
git push origin v1.0.0
```

### 3. GitHub Actions Will Automatically:

- Build Docker image (`.github/workflows/docker-image.yml`)
- Push to:
  - Docker Hub: `e1saps/wakatime-readme-updater:v1.0.0`
  - GitHub CR: `ghcr.io/apts-1547/wakatime-readme-updater:v1.0.0`
- Tag as `:latest` (for non-prerelease versions)

### 4. Update Major Version Tag (Optional)

For better user experience, maintain major version tags:

```bash
# After releasing v1.2.3, update v1 tag
git tag -fa v1 -m "Update v1 to v1.2.3"
git push origin v1 --force
```

This allows users to use `@v1` in their workflows and automatically get patch updates.

## Version Tags

The Docker workflow creates these tags:
- `v1.2.3` - Full semantic version
- `v1.2` - Major.minor version
- `v1` - Major version
- `latest` - Latest stable release (non-prerelease)
- `test` - For non-tag builds

## Action Usage

Users will reference your Action like:

```yaml
- uses: AptS-1547/wakatime-readme-updater@v1
```

GitHub will automatically use the Docker image specified in `action.yml`.
