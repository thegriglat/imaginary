build:
	docker build --target imaginary -t thegriglat/imaginary --progress=plain .

push:
	docker push thegriglat/imaginary