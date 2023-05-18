#! make

run-backend:
	cd backend && cargo run

run-frontend:
	cd frontend && npm run local

build-backend:
	cd backend && make docker-build

build-frontend:
	cd frontend && make docker-build
