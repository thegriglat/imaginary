# Imaginary
On the fly image web convert API

**Early development stage !!!**

## Environment variables

| **Variable** | **Default**    | **Description**                      |
| :----------- | :------------- | :----------------------------------- |
| `PORT`       | 8080           | Which port Imaginary will listen     |
| `WORKERS`    | number of CPUs | How many worker threads will be used |

## Query parameters

| **Variable** | **Required?** | **Allowed values** | **Description**                                            |
| :----------- | :------------ | :----------------- | :--------------------------------------------------------- |
| `url`        | yes           | valid url          | Image url to be processed                                  |
| `flip_x`     | no            | `true` or `false`  | Flip image horizontally                                    |
| `flip_y`     | no            | `true` or `false`  | Flip image vertically                                      |
| `blur`       | no            | floating number    | Blur image                                                 |
| `crop`       | no            | `x,y,width,height` | Crop image, *x*, *y* are coordinates of top left crop edge |

### Local development
```bash
cargo run 
```

## Docker image

```bash
docker build -t imaginary .
```
