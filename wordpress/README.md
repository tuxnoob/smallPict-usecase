# WordPress Local Development Environment (Docker Compose)

A comprehensive guide for installing, configuring, and running WordPress locally using **Docker** and **Docker Compose** paired with a **MariaDB** database.

---

## 📋 Table of Contents
1. [Prerequisites & Docker Installation](#1-prerequisites--docker-installation)
2. [Directory Structure](#2-directory-structure)
3. [Database & Password Configuration](#3-database--password-configuration)
4. [Generating & Configuring WordPress Salt Keys](#4-generating--configuring-wordpress-salt-keys)
5. [Running WordPress with Docker Compose](#5-running-wordpress-with-docker-compose)
6. [Completing Setup in the Browser](#6-completing-setup-in-the-browser)
7. [PHP Upload Settings (uploads.ini)](#7-php-upload-settings-uploadsini)
8. [Docker Management Commands](#8-docker-management-commands)
9. [Troubleshooting & FAQ](#9-troubleshooting--faq)

---

## 1. Prerequisites & Docker Installation

Before getting started, make sure Docker and Docker Compose are installed on your system.

### A. macOS
- **Recommended**: Download and install [Docker Desktop for Mac](https://docs.docker.com/desktop/setup/install/mac-install/) (select Apple Silicon M1/M2/M3/M4 or Intel).
- **Via Homebrew**:
  ```bash
  brew install --cask docker
  ```
- **Lightweight Alternative**: [OrbStack](https://orbstack.dev/) (`brew install orbstack`).

### B. Linux (Ubuntu / Debian)
1. Install Docker Engine using the official convenience script:
   ```bash
   curl -fsSL https://get.docker.com -o get-docker.sh
   sudo sh get-docker.sh
   ```
2. Add your current user to the `docker` group to avoid needing `sudo`:
   ```bash
   sudo usermod -aG docker $USER
   newgrp docker
   ```
3. Install the Docker Compose plugin:
   ```bash
   sudo apt-get update
   sudo apt-get install docker-compose-plugin
   ```

### C. Windows
- Download and install [Docker Desktop for Windows](https://docs.docker.com/desktop/setup/install/windows-install/) with the WSL 2 (Windows Subsystem for Linux 2) backend.

### D. Verify Installation
Run the following commands in your terminal:
```bash
docker --version
docker compose version
```
*(Note: Modern Docker Compose uses the `docker compose` command without a hyphen. If you are on an older legacy release, use `docker-compose` with a hyphen).*

---

## 2. Directory Structure

Inside `smallpict-usecase/wordpress/`, you will find:

```text
wordpress/
├── docker-compose.yml   # Container service definitions (WordPress & MariaDB)
├── uploads.ini          # Custom PHP runtime configuration (upload limits, timeouts, etc.)
├── wp-content/          # (Auto-generated on run) Local persistent volume for themes, plugins, & uploads
└── README.md            # This documentation guide
```

---

## 3. Database & Password Configuration

Open `docker-compose.yml` in your editor of choice. Locate the `environment:` section under both the `wordpress` and `db` services.

### Variable Breakdown:
1. **Service `db` (MariaDB)**:
   - `MYSQL_ROOT_PASSWORD`: The root password for the MariaDB database instance.
   - `MYSQL_USER`: Dedicated non-root user (optional, e.g. `wp_user`).
   - `MYSQL_PASSWORD`: Password for the dedicated non-root user.
   - `MYSQL_DATABASE`: Database name for WordPress (default: `wordpress`).

2. **Service `wordpress`**:
   - `WORDPRESS_DB_HOST`: Database service hostname (points to `db`).
   - `WORDPRESS_DB_NAME`: Must match `MYSQL_DATABASE` (`wordpress`).
   - `WORDPRESS_DB_USER`: User WordPress uses to connect to MariaDB.
     - If using `root`, `WORDPRESS_DB_PASSWORD` **must exactly match** `MYSQL_ROOT_PASSWORD`.
     - If using `MYSQL_USER`, `WORDPRESS_DB_PASSWORD` **must match** `MYSQL_PASSWORD`.

### Example Configuration:
```yaml
services:
  wordpress:
    image: wordpress:latest
    container_name: wordpress
    volumes:
      - ./wp-content:/var/www/html/wp-content
      - ./uploads.ini:/usr/local/etc/php/conf.d/uploads.ini
    environment:
      - WORDPRESS_DB_NAME=wordpress
      - WORDPRESS_TABLE_PREFIX=wp_
      - WORDPRESS_DB_HOST=db
      - WORDPRESS_DB_USER=root
      - WORDPRESS_DB_PASSWORD=SecretPassword123!   # <-- Replace with your secure password
      # ...
  db:
    image: mariadb:latest
    container_name: db
    volumes:
      - db_data:/var/lib/mysql
    environment:
      - MYSQL_ROOT_PASSWORD=SecretPassword123!     # <-- Must EXACTLY match WORDPRESS_DB_PASSWORD
      - MYSQL_USER=wp_user                         # (Optional)
      - MYSQL_PASSWORD=SecretPassword123!          # (Optional)
      - MYSQL_DATABASE=wordpress
    restart: always
```

---

## 4. Generating & Configuring WordPress Salt Keys

### Why Are WordPress Salt Keys Important?
WordPress Salt Keys are cryptographic random strings used to encrypt login cookies, protect user authentication sessions, and defend against session hijacking and CSRF attacks.

### A. How to Generate Salt Keys
Generate cryptographically secure random keys directly from the official WordPress API:
- **In Browser**: Visit [https://api.wordpress.org/secret-key/1.1/salt/](https://api.wordpress.org/secret-key/1.1/salt/)
- **In Terminal**:
  ```bash
  curl -s https://api.wordpress.org/secret-key/1.1/salt/
  ```

The generated output will look like this:
```php
define('AUTH_KEY',         'f)u5W8_p-e]Q...[random]');
define('SECURE_AUTH_KEY',  'X$!pL09_a~1...[random]');
define('LOGGED_IN_KEY',    '7#9kM^8v!z...[random]');
define('NONCE_KEY',        'k*P_19d%a...[random]');
define('AUTH_SALT',        'z@00_Kx#2...[random]');
define('SECURE_AUTH_SALT', 'qW!98_#m...[random]');
define('LOGGED_IN_SALT',   '1!x_Op...[random]');
define('NONCE_SALT',       '98&!_L...[random]');
```

### B. Adding Salt Keys to `docker-compose.yml`
Copy the random string value inside the quotes for each key and paste it into `docker-compose.yml`:

```yaml
    environment:
      - WORDPRESS_AUTH_KEY=f)u5W8_p-e]Q...
      - WORDPRESS_SECURE_AUTH_KEY=X$$!pL09_a~1...
      - WORDPRESS_LOGGED_IN_KEY=7#9kM^8v!z...
      - WORDPRESS_NONCE_KEY=k*P_19d%a...
      - WORDPRESS_AUTH_SALT=z@00_Kx#2...
      - WORDPRESS_SECURE_AUTH_SALT=qW!98_#m...
      - WORDPRESS_LOGGED_IN_SALT=1!x_Op...
      - WORDPRESS_NONCE_SALT=98&!_L...
```

> [!WARNING]
> **IMPORTANT: Dollar Sign (`$`) Handling in Docker Compose**  
> Docker Compose interprets `$` as environment variable interpolation. If your generated salt contains a `$`, Docker Compose may evaluate it as an empty variable or throw an error.  
> **Solution**: Escape every `$` into `$$` (e.g. `key$123` becomes `key$$123`), or simply re-run the `curl` generator command until you get a set without `$` characters.

---

## 5. Running WordPress with Docker Compose

1. Navigate to the WordPress directory:
   ```bash
   cd smallpict-usecase/wordpress
   ```

2. Start the containers in detached mode:
   ```bash
   docker compose up -d
   ```
   *(or `docker-compose up -d` for legacy versions)*

3. Verify that the containers are running:
   ```bash
   docker compose ps
   ```
   You should see both services running:
   - `wordpress`: State `Up`, port mapping `0.0.0.0:8080->80/tcp`.
   - `db`: State `Up`, port `3306/tcp`.

4. (Optional) Stream container logs to verify initial startup:
   ```bash
   docker compose logs -f
   ```
   *Press `Ctrl + C` to stop viewing the logs.*

---

## 6. Completing Setup in the Browser

Once the containers are running:

1. Open your web browser and navigate to:
   ```text
   http://localhost:8080
   ```
2. Follow the **WordPress Installation Wizard**:
   - **Language**: Select your preferred language, then click **Continue**.
   - **Site Title**: Enter your site's name (e.g., *SmallPict WordPress Test*).
   - **Username**: Choose an admin username (e.g., *admin*).
   - **Password**: Create a secure admin password (save this credentials).
   - **Your Email**: Enter your email address.
   - Click **Install WordPress**.
3. Click **Log In** and enter your newly created admin credentials.
4. The WordPress Admin Dashboard is now ready at:
   ```text
   http://localhost:8080/wp-admin
   ```

---

## 7. PHP Upload Settings (`uploads.ini`)

The `uploads.ini` file in `wordpress/` is mounted directly into the PHP configuration directory of the WordPress container:

```ini
file_uploads = On
memory_limit = 10M
upload_max_filesize = 10M
post_max_size = 20M
max_execution_time = 600
```

### Increasing Upload Size Limits
If you need to test large media uploads with the SmallPict plugin:
1. Edit `uploads.ini`:
   ```ini
   file_uploads = On
   memory_limit = 256M
   upload_max_filesize = 64M
   post_max_size = 128M
   max_execution_time = 600
   ```
2. Restart the WordPress container to apply changes:
   ```bash
   docker compose restart wordpress
   ```

---

## 8. Docker Management Commands

| Command | Description |
| :--- | :--- |
| `docker compose up -d` | Start all services in the background |
| `docker compose ps` | Check the running status of containers |
| `docker compose logs -f wordpress` | Stream real-time logs from the WordPress service |
| `docker compose logs -f db` | Stream real-time logs from the MariaDB database service |
| `docker compose stop` | Stop running containers without removing data |
| `docker compose start` | Start previously stopped containers |
| `docker compose restart` | Restart all containers |
| `docker compose down` | Stop and remove containers and network |
| `docker compose down -v` | **CAUTION**: Stop containers and permanently delete the database volume (`db_data`) for a clean reset |
| `docker compose exec wordpress bash` | Open an interactive bash shell inside the WordPress container |
| `docker compose exec db mariadb -u root -p` | Access the MariaDB interactive SQL console |

---

## 9. Troubleshooting & FAQ

### 1. Port 8080 Already Allocated
If you encounter this error:
```text
Bind for 0.0.0.0:8080 failed: port is already allocated
```
**Cause**: Another application or local service on your machine is using port `8080`.  
**Solution**: Open `docker-compose.yml`, change the host port mapping under `wordpress.ports` from `8080:80` to another open port (e.g. `8088:80`):
```yaml
    ports:
      - 8088:80
```
Then access WordPress at `http://localhost:8088`.

---

### 2. Error Establishing a Database Connection
**Causes**:
- During initial boot, MariaDB takes 10–20 seconds to initialize its default tables and users. WordPress might attempt to connect before the database is ready.
- Mismatch between `WORDPRESS_DB_PASSWORD` and `MYSQL_ROOT_PASSWORD`.

**Solutions**:
1. Wait 10–20 seconds and refresh your browser.
2. If it persists, inspect database logs:
   ```bash
   docker compose logs db
   ```
3. Ensure `WORDPRESS_DB_PASSWORD` and `MYSQL_ROOT_PASSWORD` in `docker-compose.yml` match exactly.

---

### 3. File Permissions Issue on `wp-content`
If WordPress cannot install plugins or themes locally due to directory permissions:
- **Cause**: Apache inside the container runs as `www-data` (UID 33).
- **Solution**: On your Linux/macOS host machine, adjust directory ownership to UID `33`:
  ```bash
  sudo chown -R 33:33 wp-content
  ```
