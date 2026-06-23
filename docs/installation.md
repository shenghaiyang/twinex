# Installation

### Via Homebrew

```sh
brew install shenghaiyang/tap/twinex-cli
```

### Via Shell Script

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/shenghaiyang/twinex/releases/latest/download/twinex-cli-installer.sh | sh
```

### Via PowerShell Script

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/shenghaiyang/twinex/releases/latest/download/twinex-cli-installer.ps1 | iex"
```

## Via Cargo

```sh
cargo install twinex-cli
```

### Download Manually

Download the latest release for your platform from the [GitHub Releases page](https://github.com/shenghaiyang/twinex/releases/latest), extract the archive, and place the `twinex` binary in your `PATH`.

## Build from Source

```sh
git clone https://github.com/shenghaiyang/twinex.git
cd twinex
cargo build --release
```

The compiled binary will be located at `target/release/twinex`.