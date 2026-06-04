# Lockdin Backend

 > *Pick a room. Lock in.*

Lockdin is a free, mobile-first live study accountability platform built specifically for South African high school students. Students join live subject-specific rooms — Pure Mathematics Grade 11, Life Sciences Grade 12 — and study alongside peers in real time. No messages. No social feeds. No distractions.

The mechanism is simple. Accountability through presence.

---

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Environment Variables](#environment-variables)
  - [Running the Backend](#running-the-backend)
- [API Reference](#api-reference)
- [Database](#database)
- [Caching Strategy](#caching-strategy)
- [Authentication](#authentication)
- [Contributing](#contributing)
- [Roadmap](#roadmap)

---
## Overview

Lockdin addresses a documented crisis in South African secondary education — academic isolation. With only 11% of South African homes having a dedicated study space and 40% of learners dropping out between Grade 10 and Grade 12, the absence of peer study environments is a measurable contributor to underperformance.

Lockdin applies the psychological principle of body doubling — the proven effect that working in the presence of others improves focus and task completion — to the context of the South African learner. Anywhere. Any device. Any time.

**Key characteristics:**
- CAPS and IEB aligned — Grade 8 through Grade 12
- Zero private messaging by design
- Avatar-based presence — student identity protected
- Real-time participant counts per room
- Subject category filtering — STEM, Commerce, Humanities, Creative Arts
- Notification Engineering — personalised study reminders and exam countdown campaigns

Lockdin is a subsidiary product of [The Future Academy](https://thefutureacademy.co.za) — an EdTech company providing affordable supplementary education to South African high school students.

---

## Architecture

Lockdin follows a clean layered architecture across both the backend and mobile frontend.

```
┌─────────────────────────────────────────────────────┐
│                  React Native Client                 │
│         Expo Router · Zustand · TanStack Query       │
└─────────────────────┬───────────────────────────────┘
                      │ HTTPS · JWT
┌─────────────────────▼───────────────────────────────┐
│                  Actix-web API                       │
│              Rust · REST · Middleware                │
├──────────────────────────────────────────────────────┤
│   Auth Domain  │  Users Domain  │  Rooms Domain      │
│   Notifications Domain  │  (Rooms → LiveKit)         │
└──────┬──────────────────────────────────┬────────────┘
       │                                  │
┌──────▼──────┐                  ┌────────▼────────────┐
│  PostgreSQL │                  │       Redis          │
│  (Neon)     │                  │   Participant Counts │
│  Source of  │                  │   Room List Cache    │
│  Truth      │                  │   OTP Storage        │
└─────────────┘                  └─────────────────────┘
```

**Data flow for room discovery:**
1. Client hits `GET /api/rooms` with optional grade and category filters
2. JWT middleware validates access token, extracts `user_id`
3. Rooms service checks Redis cache for the specific filter combination
4. Cache hit — returns immediately from Redis
5. Cache miss — queries PostgreSQL, merges live participant counts from Redis, caches result with 60 second TTL, returns to client

## Tech Stack

### Backend — Rust
| Technology | Purpose |
|---|---|
| Actix-web | High-performance async web framework |
| SQLx | Async type-safe PostgreSQL queries |
| Redis (`redis-rs`) | In-memory caching and participant counts |
| jsonwebtoken | JWT generation and validation |
| Chrono | Timestamp handling |
| Serde | JSON serialisation and deserialisation |
| UUID v7 | Time-ordered unique identifiers |
| Resend | Transactional email — OTP delivery |
| Tokio | Async runtime |

### Infrastructure
| Service | Purpose |
|---|---|
| PostgreSQL on Neon | Primary relational database |
| Redis (Upstash) | Caching and ephemeral data |
| Render | Backend hosting |
| Cloudflare | CDN, DDoS protection, DNS |
| Resend | Email delivery via thefutureacademy.co.za domain |
| Google Play | Android app distribution |

### Project Structure
### Backend
```
lockdin-api/
├── src/
│   ├── main.rs
│   ├── config/
│   │   ├── server.rs          # Actix-web server configuration
│   │   ├── routes.rs          # Route registration
│   │   ├── cors.rs            # CORS policy
│   │   └── state.rs           # AppState — db, redis, jwt_secret
│   ├── domains/
│   │   ├── auth/
│   │   │   ├── mod.rs
│   │   │   ├── handlers.rs    # signup, login, verify_otp, logout
│   │   │   ├── service.rs     # business logic, token generation
│   │   │   ├── repository.rs  # database operations
│   │   │   └── models.rs      # request/response structs, Claims
│   │   ├── users/
│   │   │   ├── mod.rs
│   │   │   ├── handlers.rs    # get_me, update_profile
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   └── models.rs      # User struct
│   │   ├── rooms/
│   │   │   ├── mod.rs
│   │   │   ├── handlers.rs    # get_rooms, join_room, leave_room
│   │   │   ├── service.rs     # room logic, cache management
│   │   │   ├── repository.rs  # postgres queries with full-text search
│   │   │   └── models.rs      # Room, RoomQueryParams, SubjectCategory
│   │   └── notifications/
│   │       ├── mod.rs
│   │       ├── service.rs     # send_otp_email, send_push_notification
│   │       └── templates.rs   # email copy and message templates
│   └── infra/
│       ├── database.rs        # PostgreSQL pool initialisation
│       ├── redis.rs           # Redis ConnectionManager initialisation
│       ├── env_vars.rs        # environment variable loading
│       └── middleware/
│           └── auth.rs        # JWT validation middleware, verify_token
├── migrations/
│   ├── 001_create_users_table.sql
│   ├── 002_create_otps_table.sql
│   ├── 003_create_refresh_tokens_table.sql
│   ├── 004_create_rooms_table.sql
│   └── 005_add_rooms_indexes.sql
└── Cargo.toml
```

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.75+
- [SQLx CLI](https://github.com/launchbadge/sqlx) — `cargo install sqlx-cli`
- PostgreSQL 15+
- Redis 7+

---

### Environment Variables

**Backend — create `.env` in the backend root:**

```env
DATABASE_URL=postgres://username:password@localhost:5432/lockdin
REDIS_URL=redis://127.0.0.1:6379
JWT_SECRET=your_jwt_secret_generated_with_openssl_rand_base64_64
RESEND_API_KEY=your_resend_api_key
APP_HOST=127.0.0.1
APP_PORT=8080
```

Generate your JWT secret:
```bash
openssl rand -base64 64
```

**Mobile — create `.env` in the app root:**

```env
EXPO_PUBLIC_API_URL=http://10.0.2.2:8080/api
```

> Note: `10.0.2.2` is the Android emulator's address for `localhost` on the host machine.

---

### Running the Backend

```bash
# Clone the repository
git clone https://github.com/thefutureacademysa/lockdin1.0.0.git
cd lockdin1.0.0/backend

# Install dependencies
cargo build

# Run database migrations
sqlx migrate run

# Start the development server
cargo run
```

The API will be available at `http://127.0.0.1:8080`.

---

## API Reference

All protected endpoints require a valid JWT access token in the Authorization header:

```
Authorization: Bearer <access_token>
```

### Authentication

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| POST | `/api/auth/signup` | Public | Create account, trigger OTP |
| POST | `/api/auth/login` | Public | Login with email, trigger OTP |
| POST | `/api/auth/verify-otp` | Public | Verify OTP, receive tokens |
| POST | `/api/auth/refresh` | Public | Exchange refresh token for new access token |
| POST | `/api/auth/logout` | Protected | Revoke refresh token |

### Users

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| GET | `/api/users/me` | Protected | Get authenticated user profile |
| PUT | `/api/users/me` | Protected | Update profile |

### Rooms

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| GET | `/api/rooms` | Protected | Get all rooms with optional filters |
| POST | `/api/rooms/:id/join` | Protected | Join a room, increment participant count |
| POST | `/api/rooms/:id/leave` | Protected | Leave a room, decrement participant count |

**Room query parameters:**

| Parameter | Type | Description |
|---|---|---|
| `grade` | `integer` | Filter by grade (8–12) |
| `category` | `string` | Filter by category: `stem`, `commerce`, `humanities`, `creative_arts` |
| `search` | `string` | Full-text search across room name and subject |

---

## Database

Lockdin uses PostgreSQL as the primary relational database via SQLx with compile-time query verification.

### Schema Overview

**users**
```sql
id VARCHAR PRIMARY KEY,
full_name VARCHAR NOT NULL,
school_name VARCHAR NOT NULL,
grade VARCHAR NOT NULL,
email VARCHAR UNIQUE NOT NULL,
avatar_url VARCHAR,
is_verified BOOLEAN NOT NULL DEFAULT FALSE,
created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
```

**rooms**
```sql
id VARCHAR PRIMARY KEY,
name VARCHAR NOT NULL,
subject VARCHAR NOT NULL,
grade INT NOT NULL,
category VARCHAR NOT NULL,
is_live BOOLEAN NOT NULL DEFAULT FALSE,
created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
```

**refresh_tokens**
```sql
id VARCHAR PRIMARY KEY,
user_id VARCHAR NOT NULL REFERENCES users(id),
token VARCHAR NOT NULL,
expires_at TIMESTAMPTZ NOT NULL,
created_at TIMESTAMPTZ NOT NULL,
revoked BOOLEAN NOT NULL DEFAULT FALSE
```

### Search Indexes

Lockdin uses PostgreSQL full-text search with trigram fallback for fast room discovery:

```sql
-- Full-text search across name and subject
CREATE INDEX idx_rooms_search ON rooms 
USING GIN(to_tsvector('english', name || ' ' || subject));

-- Trigram indexes for partial matching
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX idx_rooms_name_trgm ON rooms USING GIN(name gin_trgm_ops);
CREATE INDEX idx_rooms_subject_trgm ON rooms USING GIN(subject gin_trgm_ops);
```

---

## Caching Strategy

Lockdin uses Redis for two distinct purposes:

### Room List Caching

The full rooms list — including live participant counts — is cached in Redis with a 60 second TTL. Cache keys are scoped by filter combination:

```
rooms:all
rooms:grade:{grade}
rooms:grade:{grade}:category:{category}
rooms:search:{query}
```

Cache is invalidated on every `join_room` and `leave_room` event to ensure participant counts remain accurate within one cache cycle.

### Live Participant Counts

Participant counts are owned entirely by Redis — not PostgreSQL. Each room has a dedicated key:

```
room:{room_id}:participants  →  integer
```

- `INCR` on join — atomic, race condition safe
- `DECR` on leave — atomic
- TTL of 86400 seconds (24 hours) — handles disconnected clients
- PostgreSQL fallback if Redis is unavailable

---

## Authentication

Lockdin uses a phone number OTP authentication flow — no passwords.

**Signup flow:**
1. Client submits name, school, grade, email
2. User created with `is_verified: false`
3. 6-digit OTP generated, stored in PostgreSQL with 10 minute expiry
4. OTP delivered via email through Resend from `@thefutureacademy.co.za`
5. Client submits OTP code
6. OTP validated — `is_verified` flipped to `true`
7. JWT access token (60 minute expiry) and refresh token (30 day expiry) issued
8. Refresh token stored in PostgreSQL — revocable on logout or security incident

**Token strategy:**
- Access token — short-lived JWT, lives in Zustand memory only, never persisted to disk
- Refresh token — opaque 64-character alphanumeric string, persisted in Expo SecureStore, stored in PostgreSQL for revocation support
- Silent refresh — axios interceptor catches 401 responses, exchanges refresh token for new access token transparently

---

## Roadmap

- [x] Auth flow — signup, OTP verification, JWT, refresh tokens
- [x] Discover Rooms screen with real-time participant counts
- [x] Room search with full-text and trigram support
- [x] Redis caching layer
- [ ] Onboarding flow — subject selection, location, welcome screen
- [ ] Rooms API wired to frontend via TanStack Query
- [ ] LiveKit WebRTC integration — live video presence
- [ ] Avatar generation — selfie to avatar via Ready Player Me
- [ ] Notification Engineering — exam countdowns, study reminders
- [ ] Lockdin Assist — on-demand tutoring network
- [ ] Google Play Store release
- [ ] iOS release

---

## About

Lockdin is built and maintained by the team at **The Future Academy** — an EdTech company on a mission to deliver quality education to every South African student regardless of postcode, income, or circumstance.

*It is not just about school. It is about accountability in life itself. Start here.*


