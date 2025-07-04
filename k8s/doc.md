# Kubernetes & Nginx Configuration Documentation

This document explains the purpose and details of each file in the `k8s/` directory and the `nginx.conf` used in your project. It is designed to help you understand how your local/offline deployment works and why each component is necessary.

---

## 1. `k8s/config.yaml`
- **What:** Defines a Kubernetes ConfigMap and Secret.
- **Why:**
  - **ConfigMap** stores non-sensitive environment variables (e.g., JWT_SALT, JWT_EXPIRATION, DATABASE_URL, POSTGRES_DB, POSTGRES_USER).
  - **Secret** stores sensitive values (e.g., JWT_SECRET, POSTGRES_PASSWORD).
- **How:**
  - These are mounted as environment variables in your backend and Postgres pods, keeping secrets secure and configuration flexible.

---

## 2. `k8s/postgres.yaml`
- **What:** Deploys a Postgres database with persistent storage.
- **Why:**
  - Provides a database for your backend API.
  - Uses a PersistentVolumeClaim to ensure data is not lost if the pod restarts.
  - Credentials are sourced from ConfigMap/Secret for security and consistency.
- **How:**
  - The Deployment runs the official Postgres image.
  - The Service exposes Postgres internally to other pods (not to the outside world).

---

## 3. `k8s/backend.yaml`
- **What:** Deploys your Rust backend API.
- **Why:**
  - Handles authentication, user management, and API logic.
  - Uses environment variables from ConfigMap/Secret for DB connection and JWT config.
  - Exposes port 3000 internally for other services and for Ingress routing.
- **How:**
  - The Deployment runs your backend Docker image (from GHCR).
  - The Service exposes the backend to the cluster.

---

## 4. `k8s/frontend.yaml`
- **What:** Deploys your React frontend app.
- **Why:**
  - Serves the user interface for your application.
  - Uses Nginx to serve static files and proxy API requests to the backend.
  - Exposes port 80 internally for Ingress routing.
- **How:**
  - The Deployment runs your frontend Docker image (from GHCR).
  - The Service exposes the frontend to the cluster.

---

## 5. `k8s/ingress.yaml`
- **What:** Configures Ingress for pretty URLs and external access.
- **Why:**
  - Routes external HTTP requests to the correct service based on the URL path.
  - `/api` requests go to the backend; all other requests go to the frontend.
  - Allows you to access your app at a single IP/domain with clean URLs.
- **How:**
  - Requires an Ingress controller (e.g., nginx) to be installed in your cluster.
  - The rules map `/api` to the backend and `/` to the frontend.

---

## 6. `auth_api_frontend/nginx.conf`
- **What:** Nginx configuration for the frontend container.
- **Why:**
  - Serves the React app's static files.
  - Proxies `/api/` requests to the backend service (for local development or if you want to bypass Ingress for API calls inside the cluster).
  - Handles SPA routing by redirecting all non-file requests to `index.html`.
- **How:**
  - The `location /api/` block proxies API requests to the backend.
  - The `location /` block ensures React Router works by serving `index.html` for unknown paths.

---

## **How They Work Together**
- **ConfigMap/Secret** provide configuration and secrets to your pods securely.
- **Postgres** stores your application's data.
- **Backend** connects to Postgres and exposes API endpoints.
- **Frontend** serves the UI and proxies API calls to the backend.
- **Ingress** exposes your app to the outside world with clean URLs and routes traffic to the correct service.
- **Nginx (frontend)** ensures smooth SPA routing and API proxying for the React app.

---

## **Why This Setup?**
- **Security:** Secrets are not hardcoded in images or manifests.
- **Flexibility:** Configurations can be changed without rebuilding images.
- **Scalability:** Each component can be scaled independently.
- **Developer Experience:** Clean URLs, SPA support, and easy local/offline development.

---
