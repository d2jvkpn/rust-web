#! make

run-backend:
	cd backend && cargo run

run-frontend:
	cd frontend && npm run local

build-backend:
	BuildLocal=true bash deployments/build_backend.sh dev 

build-frontend:
	BuildLocal=true bash deployments/build_frontend.sh dev
