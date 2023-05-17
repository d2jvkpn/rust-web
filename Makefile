build-backend:
	BuildLocal=true bash deployments/build_backend.sh dev 

build-frontend:
	BuildLocal=true bash deployments/build_frontend.sh dev
