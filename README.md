# Imaginary
On the fly image web convert API

**Early development stage !!!**

## Environment variables

| **Variable** | **Default**    | **Description**                      |
| :----------- | :------------- | :----------------------------------- |
| `PORT`       | 8080           | Which port Imaginary will listen     |
| `WORKERS`    | number of CPUs | How many worker threads will be used |

## Query parameters

| **Variable**     | **Allowed values**          | **Description**                                                                                  |
| :--------------- | :-------------------------- | :----------------------------------------------------------------------------------------------- |
| `url` (required) | valid url                   | Image url to be processed                                                                        |
| `flip_x`         | `true` or `false`           | Flip image horizontally                                                                          |
| `flip_y`         | `true` or `false`           | Flip image vertically                                                                            |
| `grayscale`      | `true` or `false`           | Make image grayscale                                                                             |
| `blur`           | floating number             | Blur image                                                                                       |
| `crop`           | `x,y,width,height`          | Crop image, *x*, *y* are coordinates of top left crop edge                                       |
| `rotate`         | `90` or `180`or `270`       | Rotate image                                                                                     |
| `format`         | `png` or `jpeg` or `jpeg:n` | Convert image to desired format. Configure JPEG quality with *n*, default is *JPEG (quality 95)* |

### Local development
```bash
cargo run 
```

## Docker image

```bash
docker build -t imaginary .
```
