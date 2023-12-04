CONTAINER_NAME=rust-git-stats

build-dev-container:
	docker build -t $(CONTAINER_NAME) .

attach-dev-container: build-dev-container
	docker run -it --rm -v $(PWD):/home/app $(CONTAINER_NAME) bash