# Desktop application controls, dialogs, menus, and accessibility labels

# Navigation destinations
nav-calculate = Calculate
    .description = Calculate checksums for files and directories
nav-verify = Verify
    .description = Verify files against expected digests or manifests
nav-generate = Generate
    .description = Create checksum manifests for files
nav-settings = Settings
    .description = Configure algorithms, behavior, and appearance

# Calculate view
calculate-drop-zone = Drag and drop files or folders here
    .hint = Or click to browse
calculate-add-files = Add Files
calculate-add-folder = Add Folder
calculate-clear-all = Clear All
calculate-start = Calculate Checksums

# Verify view
verify-target-input = Target File
    .placeholder = Select or drop a file to verify
verify-digest-input = Expected Checksum
    .placeholder = Paste expected hexadecimal digest here
verify-manifest-input = Checksum Manifest
    .placeholder = Select or drop a checksum manifest file (.sfv, .sha256, etc.)
verify-action-compare = Compare
verify-action-verify-manifest = Verify Manifest

# Results table columns and headers
table-column-name = File Name
table-column-path = Path
table-column-size = Size
table-column-algorithm = Algorithm
table-column-status = Status
table-column-digest = Checksum
table-column-actions = Actions

# Settings view
settings-title = Application Settings
settings-algorithms = Enabled Algorithms
settings-appearance = Appearance
settings-theme-auto = System Default
settings-theme-light = Light
settings-theme-dark = Dark
settings-language = Language
settings-reset = Reset to Defaults

# Accessibility labels
a11y-progress-label = Checksum calculation progress
a11y-results-table = Checksum results table
a11y-copy-digest = Copy digest for { $path }

# RapidCRC feature actions
action-crc-into-filename = CRC into Filename
action-save-manifest = Export Manifest
action-fold-all = Fold Hashes
action-expand-all = Expand Hashes
auto-calculate-label = Auto-calculate on drop

