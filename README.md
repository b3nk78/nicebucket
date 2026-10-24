<div align="center">
  <img src="./assets/logo.png" alt="Logo of sweetbucket" width="33%">
  <h1>sweetbucket</h1>
  <p align="center"><strong>v0.1.0</strong> — A fast, private, open-source S3 GUI built with Tauri & React.</p>
  <p align="center"><em>sweetbucket is an enhanced fork of <a href="https://github.com/nicebucket-org/nicebucket">nicebucket</a> with AWS STS AssumeRole & MFA support, bulk transfer optimizations, progress tracking, and refined UI/UX.</em></p>
</div>

---

# About

**sweetbucket** is a lightweight, cross-platform desktop S3 client designed for everyday productivity without the clutter of the AWS console or the overhead of Electron.

It originated as a fork of [nicebucket](https://github.com/nicebucket-org/nicebucket) to address enterprise AWS IAM requirements (AssumeRole, MFA, master-account access restrictions), solve bulk transfer performance issues on large buckets, and provide responsive progress indicators.

## Features

- **Browse any S3-compatible bucket** (AWS S3, Cloudflare R2, MinIO, LocalStack, Custom S3 endpoints)
- **Advanced AWS IAM Security**:
  - Multi-Factor Authentication (MFA) with STS Session Tokens
  - Role assumption (`sts:AssumeRole`) with cross-account support
  - Custom region configuration for assumed roles
- **High-performance Bulk Transfers**:
  - Controlled parallel processing (Tokio Semaphore) avoiding memory/network spikes on 1,000+ objects
  - Native S3 Batch Deletion (up to 1,000 keys per HTTP request)
  - Floating dynamic progress overlay with live counters, percentages, and completion notices
- **File Management**:
  - Upload & download single or multiple files
  - Download entire folders as ZIP archives
  - Create, move, and delete folders/files
  - Preview text, images, and documents without downloading
- **Connection Profiles**:
  - Edit, save, and manage multiple profiles safely stored in your OS keyring

---

# Changelog (since fork from nicebucket)

### Version 0.1.0

#### 🔐 AWS Security & IAM Enhancements
- **MFA Authentication**: Added support for AWS Multi-Factor Authentication (MFA serial ARN + token) when initiating sessions.
- **STS AssumeRole**: Added complete support for assuming IAM roles (`sts:AssumeRole`) using credentials from a master connection.
- **Master Account Access Tolerance**: Gracefully handled restricted IAM policies where master accounts have AssumeRole rights but restricted root `s3:ListAllMyBuckets` permissions.
- **Configurable STS Region**: Added optional AWS region configuration for assumed role sessions, eliminating endpoint generation issues.
- **Convenient MFA Prompt**: MFA dialog now supports pressing the `Enter` key to submit without requiring a mouse click.

#### ⚡ Performance & Scalability (Large Buckets)
- **Batch Object Deletion**: Replaced individual single-key HTTP delete calls with native S3 Batch `delete_objects` (batches of up to 1,000 objects), drastically accelerating bulk removals and eliminating network bottlenecks.
- **Concurrency Limiting (Tokio Semaphore)**: Capped concurrent upload and download tasks (8 parallel workers) to prevent out-of-memory errors and socket exhaustion when transferring hundreds or thousands of files.
- **Endpoint Normalization**: Fixed empty region fallback logic preventing invalid S3 endpoints like `https://s3..amazonaws.com`.

#### 📊 UI & User Experience
- **Live Transfer Progress Bar (`TransferProgressOverlay`)**:
  - Added a non-intrusive floating progress widget at the bottom right.
  - Displays real-time file counters (`X / Y`), completion percentage, and animated bar during upload and download operations.
  - Shows success notification with auto-dismissal once transfer completes.
- **Connection Profile Edition**:
  - Added an "Edit" button next to saved connections to load and update existing configurations without manual re-entry.
  - Added visual edit-mode banner with a one-click cancel action.
- **Responsive Layout Fixes**: Fixed CSS overflow issues where action buttons were pushed out of frame on narrow or resized windows.
- **Friendly Error Reporting**: Replaced raw "ERROR" indicators with helpful contextual messages when buckets are inaccessible or permissions are missing.

---

## Installation

### Download
Download the latest binaries for Windows, macOS, or Linux from the [Releases](https://github.com/b3nk78/nicebucket/releases) section.

#### Linux (Wayland)
If you encounter Wayland or scaling glitches with the AppImage:
```bash
LD_PRELOAD=/usr/lib/libwayland-client.so GDK_SCALE=1 ./sweetbucket.AppImage
```

---

## Getting Started

1. Launch **sweetbucket**
2. Add your credentials (S3, Cloudflare R2, Custom endpoint, or AssumeRole configuration)
3. If using MFA or AssumeRole, enter your token when prompted
4. Browse, upload, download, and manage your buckets smoothly

---

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v20+)
- [Tauri CLI v2](https://v2.tauri.app/reference/cli/)

### Local Setup

```bash
git clone https://github.com/b3nk78/nicebucket.git sweetbucket
cd sweetbucket
npm install
npm run tauri dev
```

### Build

```bash
# Frontend build
npm run build

# Production desktop application
npm run tauri build
```

---

## Tech Stack

- **Frontend**: React 19 + TypeScript + Tailwind CSS v4 + Vite + Lucide Icons + TanStack Query
- **Backend**: Rust + Tauri v2 + Tokio
- **AWS Integration**: AWS SDK for Rust (`aws-sdk-s3`, `aws-sdk-sts`, `aws-config`)
- **Keyring**: `tauri-plugin-keyring`

---

## License

sweetbucket is free and open-source software licensed under the **GPLv3** (inherited from the original [nicebucket](https://github.com/nicebucket-org/nicebucket) project). See [LICENSE.md](LICENSE.md) for details.
