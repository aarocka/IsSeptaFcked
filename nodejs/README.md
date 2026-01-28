# IsSeptaFcked - Node.js Implementation

This is the original Node.js implementation of the IsSeptaFcked application.

For the full original documentation, see [README-original.md](README-original.md).

## Quick Start

```bash
npm install
npm start
```

The application will start on port 5000 by default.

## Docker

### With Docker Compose:
```bash
docker-compose build && docker-compose up
```

### With Docker:
```bash
docker build -t septa . && docker run -e TZ=EST5EDT -p 5000:5000 -it septa
```

Visit http://localhost:5000/

## Project Structure

- `lib/` - Core application modules
  - `septa/rr/` - Regional Rail API and logic
  - `septa/bus/` - Bus API and logic
  - `sfw.js` - Safe-for-work filtering
- `routes/` - HTTP route handlers
- `views/` - Pug templates
- `public/` - Static assets (CSS, favicon, etc.)
- `web.js` - Main application entry point

## Environment Variables

- `PORT` - Port to listen on (default: 5000)
- `NODE_ENV` - Set to `production` for production mode
- `TZ` - Timezone (e.g., `EST5EDT`)

## Deployment

See [README-original.md](README-original.md) for deployment instructions for:
- Fly.io
- Heroku
- Docker

## API Endpoints

All endpoints are the same as the Rust version. See the main [README.md](../README.md) for details.
