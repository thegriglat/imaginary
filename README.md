# Imaginary

A Rust HTTP server that converts images on the fly

**Early development stage !!!**

In common image convert adds 70-200ms to request processing time.

## Query parameters

| **Variable**     | **Allowed values**          | **Description**                                                                                  |
| :--------------- | :-------------------------- | :----------------------------------------------------------------------------------------------- |
| `url` (required) | valid url                   | Image url to be processed                                                                        |
| `flip_x`         | `true` or `false`           | Flip image horizontally                                                                          |
| `flip_y`         | `true` or `false`           | Flip image vertically                                                                            |
| `grayscale`      | `true` or `false`           | Make image grayscale                                                                             |
| `blur`           | floating number > 0         | Blur image                                                                                       |
| `crop`           | `x,y,width,height`          | Crop image, *x*, *y* are coordinates of top left crop edge                                       |
| `rotate`         | `90` or `180`or `270`       | Rotate image                                                                                     |
| `format`         | `png` or `jpeg` or `jpeg:n` | Convert image to desired format. Configure JPEG quality with *n*, default is *JPEG (quality 95)* |


## Deployment

### Using Docker Hub image

```bash
# with default port
docker run -p 8080:8080 thegriglat/imaginary:latest
```

or

```bash
# with custom port
docker run -p 8081:8081 -e PORT=8081 thegriglat/imaginary:latest
```

then open in browser [http://localhost:8080/?url=https://upload.wikimedia.org/wikipedia/commons/b/b2/JPEG_compression_Example.jpg&blur=2&rotate=90](http://localhost:8080/?url=https://upload.wikimedia.org/wikipedia/commons/b/b2/JPEG_compression_Example.jpg&blur=2&rotate=90) (change port if needed)

### Build your own image

```bash
make build
```

## Environment variables

| **Variable** | **Default** | **Description**                  |
| :----------- | :---------- | :------------------------------- |
| `PORT`       | 8080        | Which port Imaginary will listen |


## Local development
```bash
cargo run 
```

If you are reading this -- you know what to do.