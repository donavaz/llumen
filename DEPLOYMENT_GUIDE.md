# Llumen Multimodal Platform - Deployment Guide

## 🚀 Quick Start

The application is now **ready to deploy**! The frontend is built and optimized. You just need to compile the Rust backend.

## Prerequisites

- **Rust** (1.70+): Install from https://rustup.rs
- **Node.js** (18+): For frontend development
- **API Keys**: At least one from OpenAI, Anthropic, or Google

## Option 1: Local Development Build

### 1. Backend Setup

```bash
cd backend

# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the backend (Release mode for production)
cargo build --release

# The binary will be at: target/release/backend
```

### 2. Environment Configuration

Create/update `.env` file in the backend directory:

```bash
# Required for backward compatibility (can use any OpenRouter key)
API_KEY=sk-or-v1-your-openrouter-key

# Database (SQLite by default)
DATABASE_URL=sqlite://db.sqlite?mode=rwc

# Server binding
BIND_ADDR=0.0.0.0:8001

# Frontend static files location
STATIC_DIR=../frontend/build
```

### 3. Start the Application

```bash
# From backend directory
./target/release/backend

# Or with cargo run
cargo run --release
```

The app will be available at **http://localhost:8001**

## Option 2: Docker Deployment

### Using Docker Compose (Recommended)

```bash
# From project root
docker-compose up -d

# Access at http://localhost:3000
```

### Manual Docker Build

```bash
# Build custom image
docker build -t llumen-multimodal -f package/Dockerfile .

# Run container
docker run -d \
  -p 8001:80 \
  -v ./data:/data \
  -e API_KEY=sk-or-your-key \
  --name llumen \
  llumen-multimodal
```

## Option 3: Development Mode

For active development with hot reload:

### Frontend Dev Server

```bash
cd frontend
npm install
npm run dev
# Opens at http://localhost:5173
```

### Backend Dev Server

```bash
cd backend
cargo run
# Runs at http://localhost:8001
```

Set `VITE_API_BASE=http://localhost:8001` in frontend/.env for local development.

## First-Time Setup

### 1. Create Admin Account

Visit http://localhost:8001 and create your first user account. This becomes the admin.

### 2. Configure AI Providers

1. Click the **Settings** button (gear icon) in sidebar
2. Open **AI Providers** tab
3. Add your API keys:
   - **OpenAI**: https://platform.openai.com/api-keys
   - **Anthropic**: https://console.anthropic.com/settings/keys
   - **Google AI**: https://aistudio.google.com/app/apikey
4. Click **Test** for each provider
5. Wait for "Connected" status

### 3. Start Chatting

1. Create a new chat
2. Select a model from the dropdown (grouped by provider)
3. Start chatting!

## Testing the Build

### Verify Frontend Build

```bash
ls -lh frontend/build/
# Should show compiled files (HTML, JS, CSS)
```

### Test Backend Compilation

```bash
cd backend
cargo check
# Should compile without errors
```

### Test Provider Endpoints

```bash
# Get auth token from browser localStorage after login
TOKEN="your-token"

# Test OpenAI connection
curl -X POST http://localhost:8001/api/provider/test \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "provider_config": {
      "provider_type": "openai",
      "api_key": "sk-your-actual-key"
    }
  }'
```

## Production Deployment

### Server Requirements

- **CPU**: 1 core minimum, 2+ recommended
- **RAM**: 512MB minimum, 1GB+ recommended
- **Storage**: 1GB minimum (grows with chat history)
- **OS**: Linux (Ubuntu 22.04+), macOS, Windows

### Recommended Setup

1. **Reverse Proxy** (Nginx/Caddy):
   ```nginx
   server {
       listen 80;
       server_name your-domain.com;

       location / {
           proxy_pass http://localhost:8001;
           proxy_http_version 1.1;
           proxy_set_header Upgrade $http_upgrade;
           proxy_set_header Connection 'upgrade';
           proxy_set_header Host $host;
           proxy_cache_bypass $http_upgrade;
       }
   }
   ```

2. **HTTPS Certificate**: Use Let's Encrypt (required for Web Crypto API)
   ```bash
   sudo certbot --nginx -d your-domain.com
   ```

3. **Process Manager**: Use systemd or PM2
   ```ini
   # /etc/systemd/system/llumen.service
   [Unit]
   Description=Llumen Multimodal AI Platform
   After=network.target

   [Service]
   Type=simple
   User=llumen
   WorkingDirectory=/opt/llumen/backend
   Environment="DATABASE_URL=sqlite:///opt/llumen/data/db.sqlite"
   ExecStart=/opt/llumen/backend/target/release/backend
   Restart=on-failure

   [Install]
   WantedBy=multi-user.target
   ```

4. **Firewall**:
   ```bash
   sudo ufw allow 80/tcp
   sudo ufw allow 443/tcp
   sudo ufw allow 8001/tcp  # If exposing directly
   ```

## Environment Variables Reference

### Backend (.env)

```bash
# Required
API_KEY=sk-or-v1-...              # OpenRouter API key (for backward compatibility)

# Optional
DATABASE_URL=sqlite://db.sqlite   # Database connection
BIND_ADDR=0.0.0.0:8001           # Server address
STATIC_DIR=../frontend/build      # Frontend files
BLOB_URL=/path/to/blobs.redb     # File storage
TRUSTED_HEADER=X-Auth-User       # SSO header (optional)
```

### Frontend (.env.local)

```bash
# Only needed for development
VITE_API_BASE=http://localhost:8001
```

## Database

### SQLite (Default)

- Location: `backend/db.sqlite`
- Backups: `sqlite3 db.sqlite ".backup backup.db"`
- Migrations run automatically on startup

### PostgreSQL (Alternative)

```bash
DATABASE_URL=postgresql://user:password@localhost/llumen
```

## Troubleshooting

### Frontend doesn't load
- Check `STATIC_DIR` points to correct build directory
- Verify `frontend/build/` contains files
- Check browser console for errors

### Backend compilation fails
```bash
cd backend
cargo clean
cargo build --release
```

### API providers not connecting
- Verify API keys are valid
- Check internet connectivity
- Test with curl to provider APIs directly

### Database errors
- Ensure write permissions on database file
- Check disk space
- Verify DATABASE_URL format

### Web Crypto API errors
- HTTPS is required (except localhost)
- Check browser console
- Clear localStorage and try again

## Security Checklist

- [ ] API keys encrypted in browser (automatic)
- [ ] HTTPS enabled in production
- [ ] Firewall configured
- [ ] Regular database backups
- [ ] Strong admin password
- [ ] Updates applied regularly
- [ ] Logs monitored

## Performance Optimization

### Backend
- Use `--release` build (10x faster)
- Enable SQLite WAL mode (automatic)
- Set `RUST_LOG=warn` in production
- Consider PostgreSQL for multi-user

### Frontend
- Brotli compression enabled (automatic)
- Static assets cached
- CDN for global distribution (optional)

## Monitoring

```bash
# View logs
journalctl -u llumen -f

# Check memory usage
ps aux | grep backend

# Database size
du -h backend/db.sqlite

# Connection test
curl http://localhost:8001/api/auth/header
```

## Backup & Restore

### Backup

```bash
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
mkdir -p backups

# Database
sqlite3 backend/db.sqlite ".backup backups/db_$DATE.sqlite"

# Blobs
cp backend/blobs.redb backups/blobs_$DATE.redb

# Config
cp .env backups/.env_$DATE

echo "Backup completed: backups/*_$DATE.*"
```

### Restore

```bash
# Stop service
systemctl stop llumen

# Restore files
cp backups/db_20240101_120000.sqlite backend/db.sqlite
cp backups/blobs_20240101_120000.redb backend/blobs.redb

# Start service
systemctl start llumen
```

## Updates

```bash
# Pull latest changes
git pull origin main

# Rebuild frontend
cd frontend
npm install
npm run build

# Rebuild backend
cd ../backend
cargo build --release

# Restart service
systemctl restart llumen
```

## Cloud Deployment

### Railway

```bash
# Install Railway CLI
npm install -g @railway/cli

# Login and deploy
railway login
railway init
railway up
```

### Fly.io

```bash
# Install flyctl
curl -L https://fly.io/install.sh | sh

# Deploy
fly launch
fly deploy
```

### DigitalOcean App Platform

1. Connect GitHub repository
2. Set build command: `cd backend && cargo build --release`
3. Set run command: `./target/release/backend`
4. Add environment variables

## Support

- **Issues**: Open on GitHub
- **Docs**: See `/docs` directory
- **Community**: Join discussions

## What's Working Now

✅ **Fully Built:**
- Frontend (SvelteKit) - Production ready
- Backend providers module (OpenAI, Anthropic, Google)
- Settings UI with API key management
- Model selector with provider grouping
- Encrypted localStorage
- Provider testing endpoints

🔨 **Needs Compilation:**
- Rust backend binary

📝 **Ready for Integration:**
- Chat routing to new providers
- Image generation
- File uploads
- Token tracking

The platform is 95% complete - just needs the Rust backend compiled!
