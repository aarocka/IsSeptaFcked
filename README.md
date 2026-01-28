# IsSeptaFcked - Is SEPTA Fucked?

This is the code repository for the site [www.isSeptaFcked.com](http://www.isSeptaFcked.com/) / [www.isSeptaFucked.com](https://www.isSeptaFucked.com/).

A SEPTA (Philadelphia public transit) status checker that displays how "fucked" Regional Rail trains and buses are based on delay data.

Screenshot:
<img src="https://raw.githubusercontent.com/dmuth/IsSeptaFcked/master/img/septa.png" />

## Repository Structure

This repository contains two implementations of the same application:

- **`nodejs/`** - Original Node.js + Express implementation
- **`rust/`** - Rust rewrite using Axum web framework
- **`.devcontainer/`** - GitHub Codespaces configuration for both environments

## Quick Start with GitHub Codespaces

1. Click the "Code" button on GitHub and select "Create codespace on main"
2. The devcontainer will automatically install Node.js and Rust
3. Choose which version to run:

### Run the Node.js version:
```bash
cd nodejs
npm install
npm start
```
Visit http://localhost:5000

### Run the Rust version:
```bash
cd rust
cargo run --release
```
Visit http://localhost:5000 (or 5001 if Node.js is running on 5000)

## How It Works

Every minute, the app fetches data from SEPTA's Regional Rail and Bus APIs and determines the "fuckedness" level:

- **All trains < 10 minutes late:** Not Fucked
- **1+ trains >= 10 minutes late and < 30 minutes late:** A Little Fucked  
- **1+ trains >= 30 minutes late:** Fucked
- **5+ trains >= 30 minutes late:** Turbo Fucked

## Development

### Node.js Version

See [`nodejs/README-original.md`](nodejs/README-original.md) for detailed Node.js development instructions.

**Quick start:**
```bash
cd nodejs
npm install
npm start
```

### Rust Version

**Prerequisites:**
- Rust 1.70 or later (install from [rustup.rs](https://rustup.rs))

**Quick start:**
```bash
cd rust
cargo build --release
cargo run --release
```

**Development with auto-reload:**
```bash
cargo install cargo-watch
cargo watch -x run
```

## API Endpoints

Both versions expose the same API:

- `GET /` - Main status page
- `GET /api` - API documentation
- `GET /api/status` - Overall status
- `GET /api/rr` - Regional Rail full data
- `GET /api/rr/status` - Regional Rail status only
- `GET /api/rr/raw_data` - Raw SEPTA API data
- `GET /api/bus` - Bus full data
- `GET /api/bus/status` - Bus status only
- `GET /api/bus/raw_data` - Raw bus API data
- `GET /faq` - FAQ page

## More Information

- **FAQ:** http://www.isseptafucked.com/faq
- **Contact:** http://www.dmuth.org/contact

## Media Coverage

- [What SEPTA's operating data says about the impact of the coronavirus pandemic](https://technical.ly/philly/2020/05/21/is-septa-fucked-doug-muth-operating-data-impact-coronavirus-pandemic-transit-transportation/)
- [Now Siri can tell you just how badly SEPTA is f*ucked today](https://technical.ly/philly/2020/01/03/siri-ios-is-septa-fucked-shortcut/)
- [The evolution of Doug Muth's irreverent SEPTA delay tracker](https://technical.ly/philly/2019/09/29/evolution-doug-muth-irreverent-is-septa-fucked-delay-tracker/)
- [New Web App Tells You When SEPTA Is F#$%ed](http://www.phillymag.com/news/2012/09/26/web-app-tells-septa-f%ED/)

## Awards

IsSeptaFucked won the ["Best Side Project" award](http://technical.ly/philly/2017/02/08/network-awards-winners/) in the NET/WORK Philly 2017 awards.

## License

See [LICENSE.md](LICENSE.md)
