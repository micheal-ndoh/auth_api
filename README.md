# Auth API Project

This project is a full-stack authentication system with a Rust (Axum) backend and a React frontend.

## Table of Contents

- [Auth API Project](#auth-api-project)
  - [Table of Contents](#table-of-contents)
  - [Local Development Setup](#local-development-setup)
    - [Prerequisites](#prerequisites)
    - [Backend Setup](#backend-setup)
    - [Database Setup (Local)](#database-setup-local)
    - [Frontend Setup](#frontend-setup)
  - [Deployment](#deployment)
    - [Backend to Railway](#backend-to-railway)
    - [Frontend to Vercel](#frontend-to-vercel)
  - [API Usage](#api-usage)

---

## Local Development Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/get-started)
- [Node.js](https://nodejs.org/en/)
- `psql` command-line tool (PostgreSQL client)

### Backend Setup

1. Navigate to the project root (`auth-api`).
2. Create a `.env` file and add the following, replacing the values as needed:

    ```env
    DATABASE_URL=postgres://authuser:yourpassword@localhost:5432/authdb
    JWT_SECRET=your_super_secret_jwt_string
    ```

3. Run the backend:

    ```bash
    cargo run
    ```

    The API will be available at `http://localhost:3000`.

### Database Setup (Local)

1. Start a PostgreSQL container using Docker:

    ```bash
    docker run --name auth-postgres -e POSTGRES_PASSWORD=yourpassword -e POSTGRES_USER=authuser -e POSTGRES_DB=authdb -p 5432:5432 -d postgres
    ```

2. Connect to the database using `psql`:

    ```bash
    psql -h localhost -U authuser -d authdb
    ```

    When prompted, enter the password (`yourpassword`).
3. Run the following SQL to create the necessary tables:

    ```sql
    CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        email VARCHAR(255) UNIQUE NOT NULL,
        firstname VARCHAR(255) NOT NULL,
        lastname VARCHAR(255) NOT NULL,
        password_hash VARCHAR(255) NOT NULL,
        role VARCHAR(50) NOT NULL
    );

    CREATE TABLE IF NOT EXISTS tokens (
        id SERIAL PRIMARY KEY,
        user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
        token VARCHAR(512) NOT NULL,
        expires_at TIMESTAMP NOT NULL
    );
    ```

4. Exit `psql` with `\q`.

### Frontend Setup

1. Navigate to the frontend directory:

    ```bash
    cd auth_api_frontend
    ```

2. Install dependencies:

    ```bash
    npm install
    ```

3. Start the development server:

    ```bash
    npm start
    ```

    The frontend will be available at `http://localhost:3001` (or another port if 3000 is taken).

---

## Deployment

### Backend to Railway

1. Push your backend code to a GitHub repository.
2. Create a new project on [Railway](https://railway.app/) and deploy from your GitHub repo.
3. Add a PostgreSQL database plugin to your Railway project.
4. **Install `psql` (PostgreSQL client):** Before you can connect to the Railway database from your local machine, you need the `psql` command-line tool.
    - On Debian/Ubuntu: `sudo apt-get install postgresql-client`
    - On macOS (with Homebrew): `brew install libpq`
5. In your backend service's **Variables** tab on Railway, set the following:
    - `DATABASE_URL`: Use the **public connection URL** provided by the Railway Postgres plugin.
    - `JWT_SECRET`: Set a strong, unique secret.
6. Connect to your Railway Postgres database using the provided `psql` command or connection string

 ```shell
PGPASSWORD=your_railway_db_password psql -h turntable.proxy.rlwy.net -U postgres -p 52084 -d railway
```

 and run the table creation SQL from the [local setup section](#database-setup-local).

1. Your backend will automatically deploy. The public URL will be available in the Railway dashboard.

### Frontend to Vercel

1. Push your frontend code (`auth_api_frontend` directory) to its own GitHub repository.
2. Create a new project on [Vercel](https://vercel.com/) and import your frontend's GitHub repo.
3. Vercel will automatically detect that it is a React app and configure the build settings.
4. In your frontend code (`src/AuthContext.tsx` or a similar config file), make sure the `API_BASE` constant points to your **deployed Railway backend URL**.
5. Deploy the project.

---

## API Usage

You can interact with the API via the Swagger UI, available at `/swagger-ui` on your running backend (e.g `http://localhost:3000/swagger-ui`).
